use crate::ast::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Ident(String),
    Number(f64),
    String(String),
    Symbol(String),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

pub fn parse(source: &str) -> Result<Program, ParseError> {
    Parser::new(source)?.parse_program()
}

pub fn try_parse(source: &str) -> Result<Program, ParseError> {
    parse(source)
}

struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    _source: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            _source: source,
        }
    }

    fn lex(mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
                continue;
            }
            if ch == '#' {
                self.skip_line_comment();
                continue;
            }
            if ch == '/' && self.peek_n(1) == Some('/') {
                self.skip_line_comment();
                continue;
            }
            if ch == '/' && self.peek_n(1) == Some('*') {
                self.skip_block_comment()?;
                continue;
            }

            let line = self.line;
            let column = self.column;

            if ch == '"' {
                tokens.push(Token {
                    kind: TokenKind::String(self.lex_string()?),
                    line,
                    column,
                });
                continue;
            }

            if ch.is_ascii_alphabetic() || ch == '_' {
                tokens.push(Token {
                    kind: TokenKind::Ident(self.lex_ident()),
                    line,
                    column,
                });
                continue;
            }

            if ch.is_ascii_digit()
                || (ch == '-' && self.peek_n(1).is_some_and(|c| c.is_ascii_digit()))
            {
                tokens.push(Token {
                    kind: TokenKind::Number(self.lex_number()?),
                    line,
                    column,
                });
                continue;
            }

            if ch == '>' && self.peek_n(1) == Some('=') {
                self.bump();
                self.bump();
                tokens.push(Token {
                    kind: TokenKind::Symbol(">=".to_string()),
                    line,
                    column,
                });
                continue;
            }

            if "{}()[],.:%/".contains(ch) {
                self.bump();
                tokens.push(Token {
                    kind: TokenKind::Symbol(ch.to_string()),
                    line,
                    column,
                });
                continue;
            }

            return Err(ParseError {
                message: format!("Unexpected character '{ch}'"),
                line,
                column,
            });
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            column: self.column,
        });
        Ok(tokens)
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.chars.get(self.pos + n).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            self.bump();
            if ch == '\n' {
                break;
            }
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), ParseError> {
        let start_line = self.line;
        let start_column = self.column;
        self.bump();
        self.bump();
        while let Some(ch) = self.peek() {
            if ch == '*' && self.peek_n(1) == Some('/') {
                self.bump();
                self.bump();
                return Ok(());
            }
            self.bump();
        }
        Err(ParseError {
            message: "Unclosed block comment".to_string(),
            line: start_line,
            column: start_column,
        })
    }

    fn lex_string(&mut self) -> Result<String, ParseError> {
        let start_line = self.line;
        let start_column = self.column;
        self.bump();
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                return Ok(out);
            }
            out.push(ch);
            self.bump();
        }
        Err(ParseError {
            message: "Unclosed string literal".to_string(),
            line: start_line,
            column: start_column,
        })
    }

    fn lex_ident(&mut self) -> String {
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                out.push(ch);
                self.bump();
            } else {
                break;
            }
        }
        out
    }

    fn lex_number(&mut self) -> Result<f64, ParseError> {
        let line = self.line;
        let column = self.column;
        let mut text = String::new();
        if self.peek() == Some('-') {
            text.push('-');
            self.bump();
        }
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                text.push(ch);
                self.bump();
            } else {
                break;
            }
        }
        if self.peek() == Some('.') {
            text.push('.');
            self.bump();
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    text.push(ch);
                    self.bump();
                } else {
                    break;
                }
            }
        }

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() {
                self.bump();
            } else {
                break;
            }
        }

        text.parse::<f64>().map_err(|_| ParseError {
            message: format!("Invalid number '{text}'"),
            line,
            column,
        })
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(source: &str) -> Result<Self, ParseError> {
        Ok(Self {
            tokens: Lexer::new(source).lex()?,
            pos: 0,
        })
    }

    fn parse_program(mut self) -> Result<Program, ParseError> {
        let mut units = None;
        let mut origin = None;
        let mut axis = None;
        let mut grid = None;
        let mut defaults = None;
        let mut render = None;
        let mut dimension_units = None;
        let mut site = None;
        let mut catalogs = Vec::new();

        loop {
            if self.check_keyword("units") {
                units = Some(self.parse_units()?);
            } else if self.check_keyword("origin") {
                origin = Some(self.parse_origin()?);
            } else if self.check_keyword("axis") {
                axis = Some(self.parse_axis()?);
            } else if self.check_keyword("grid") {
                grid = Some(self.parse_grid()?);
            } else if self.check_keyword("defaults") {
                defaults = Some(self.parse_defaults()?);
            } else if self.check_keyword("render") {
                render = Some(self.parse_render()?);
            } else if self.check_keyword("dimension_units") {
                dimension_units = Some(self.parse_dimension_units_declaration()?);
            } else if self.check_keyword("site") {
                site = Some(self.parse_site()?);
            } else if self.check_keyword("catalog") {
                catalogs.push(self.parse_catalog()?);
            } else {
                break;
            }
        }

        let plan = self.parse_plan()?;
        self.expect_eof()?;
        Ok(Program {
            node_type: node_type("Program"),
            units,
            origin,
            axis,
            grid,
            defaults,
            render,
            dimension_units,
            site,
            catalogs,
            plan,
        })
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if !matches!(token.kind, TokenKind::Eof) {
            self.pos += 1;
        }
        token
    }

    fn error_here(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            message: message.into(),
            line: self.current().line,
            column: self.current().column,
        }
    }

    fn check_keyword(&self, kw: &str) -> bool {
        matches!(&self.current().kind, TokenKind::Ident(s) if s.eq_ignore_ascii_case(kw))
    }

    fn check_symbol(&self, sym: &str) -> bool {
        matches!(&self.current().kind, TokenKind::Symbol(s) if s == sym)
    }

    fn expect_keyword(&mut self, kw: &str) -> Result<(), ParseError> {
        if self.check_keyword(kw) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_here(format!("Expected keyword '{kw}'")))
        }
    }

    fn consume_keyword(&mut self, kw: &str) -> bool {
        if self.check_keyword(kw) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_symbol(&mut self, sym: &str) -> Result<(), ParseError> {
        if self.check_symbol(sym) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_here(format!("Expected '{sym}'")))
        }
    }

    fn consume_symbol(&mut self, sym: &str) -> bool {
        if self.check_symbol(sym) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.advance().kind {
            TokenKind::Ident(s) => Ok(s),
            _ => Err(self.error_here("Expected identifier")),
        }
    }

    fn expect_number(&mut self) -> Result<f64, ParseError> {
        match self.advance().kind {
            TokenKind::Number(n) => Ok(n),
            _ => Err(self.error_here("Expected number")),
        }
    }

    fn expect_string(&mut self) -> Result<String, ParseError> {
        match self.advance().kind {
            TokenKind::String(s) => Ok(s),
            _ => Err(self.error_here("Expected string")),
        }
    }

    fn expect_eof(&mut self) -> Result<(), ParseError> {
        if matches!(self.current().kind, TokenKind::Eof) {
            Ok(())
        } else {
            Err(self.error_here("Expected end of input"))
        }
    }

    fn parse_units(&mut self) -> Result<UnitsDeclaration, ParseError> {
        self.expect_keyword("units")?;
        let raw = self.expect_ident()?.to_lowercase();
        let unit = match raw.as_str() {
            "m" => UnitType::M,
            "meters" => UnitType::Meters,
            "cm" => UnitType::Cm,
            "mm" => UnitType::Mm,
            "ft" => UnitType::Ft,
            "in" => UnitType::In,
            _ => return Err(self.error_here(format!("Unknown unit '{raw}'"))),
        };
        Ok(UnitsDeclaration {
            node_type: node_type("UnitsDeclaration"),
            unit,
        })
    }

    fn parse_origin(&mut self) -> Result<OriginDeclaration, ParseError> {
        self.expect_keyword("origin")?;
        let point = self.parse_point()?;
        Ok(OriginDeclaration {
            node_type: node_type("OriginDeclaration"),
            point,
        })
    }

    fn parse_axis(&mut self) -> Result<AxisDeclaration, ParseError> {
        self.expect_keyword("axis")?;
        self.expect_keyword("x")?;
        self.expect_symbol(":")?;
        let x = self.parse_axis_direction()?;
        self.expect_keyword("y")?;
        self.expect_symbol(":")?;
        let y = self.parse_axis_direction()?;
        Ok(AxisDeclaration {
            node_type: node_type("AxisDeclaration"),
            x,
            y,
        })
    }

    fn parse_axis_direction(&mut self) -> Result<AxisDirection, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "right" => Ok(AxisDirection::Right),
            "left" => Ok(AxisDirection::Left),
            "up" => Ok(AxisDirection::Up),
            "down" => Ok(AxisDirection::Down),
            _ => Err(self.error_here(format!("Unknown axis direction '{raw}'"))),
        }
    }

    fn parse_grid(&mut self) -> Result<GridDeclaration, ParseError> {
        self.expect_keyword("grid")?;
        Ok(GridDeclaration {
            node_type: node_type("GridDeclaration"),
            size: self.expect_number()?,
        })
    }

    fn parse_defaults(&mut self) -> Result<DefaultsDeclaration, ParseError> {
        self.expect_keyword("defaults")?;
        self.expect_symbol("{")?;
        let mut defaults = DefaultsDeclaration {
            node_type: node_type("DefaultsDeclaration"),
            ..Default::default()
        };
        while !self.consume_symbol("}") {
            if self.consume_keyword("door_width") {
                defaults.door_width = Some(self.expect_number()?);
            } else if self.consume_keyword("window_width") {
                defaults.window_width = Some(self.expect_number()?);
            } else if self.consume_keyword("floor") {
                defaults.floor = Some(self.parse_material_id()?);
            } else if self.consume_keyword("indoor_floor") {
                defaults.floor = Some(self.parse_material_id()?);
            } else if self.consume_keyword("outdoor_floor") {
                defaults.outdoor_floor = Some(self.parse_material_id()?);
            } else {
                return Err(self.error_here("Expected defaults item"));
            }
        }
        Ok(defaults)
    }

    fn parse_render(&mut self) -> Result<RenderDeclaration, ParseError> {
        self.expect_keyword("render")?;
        self.expect_symbol("{")?;
        let mut mode = RenderMode::Color;
        while !self.consume_symbol("}") {
            if self.consume_keyword("mode") {
                mode = self.parse_render_mode()?;
            } else {
                return Err(self.error_here("Expected render content"));
            }
        }
        Ok(RenderDeclaration {
            node_type: node_type("RenderDeclaration"),
            mode,
        })
    }

    fn parse_render_mode(&mut self) -> Result<RenderMode, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "color" | "colour" => Ok(RenderMode::Color),
            "draft" | "black_white" | "blackwhite" | "bw" | "b_w" => Ok(RenderMode::Draft),
            _ => Err(self.error_here(format!("Unknown render mode '{raw}'"))),
        }
    }

    fn parse_site(&mut self) -> Result<SiteDeclaration, ParseError> {
        self.expect_keyword("site")?;
        self.expect_symbol("{")?;
        let mut street = CardinalDirection::South;
        let mut hemisphere = None;
        while !self.consume_symbol("}") {
            if self.consume_keyword("street") {
                street = self.parse_cardinal()?;
            } else if self.consume_keyword("hemisphere") {
                hemisphere = Some(self.parse_hemisphere()?);
            } else {
                return Err(self.error_here("Expected site item"));
            }
        }
        Ok(SiteDeclaration {
            node_type: node_type("SiteDeclaration"),
            street,
            hemisphere,
        })
    }

    fn parse_catalog(&mut self) -> Result<CatalogDeclaration, ParseError> {
        self.expect_keyword("catalog")?;
        Ok(CatalogDeclaration {
            node_type: node_type("CatalogDeclaration"),
            path: self.expect_string()?,
        })
    }

    fn parse_cardinal(&mut self) -> Result<CardinalDirection, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "north" => Ok(CardinalDirection::North),
            "south" => Ok(CardinalDirection::South),
            "east" => Ok(CardinalDirection::East),
            "west" => Ok(CardinalDirection::West),
            _ => Err(self.error_here(format!("Unknown direction '{raw}'"))),
        }
    }

    fn parse_hemisphere(&mut self) -> Result<Hemisphere, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "north" => Ok(Hemisphere::North),
            "south" => Ok(Hemisphere::South),
            _ => Err(self.error_here(format!("Unknown hemisphere '{raw}'"))),
        }
    }

    fn parse_plan(&mut self) -> Result<PlanDefinition, ParseError> {
        self.expect_keyword("plan")?;
        let name = if matches!(self.current().kind, TokenKind::String(_)) {
            self.expect_string()?
        } else {
            "unnamed".to_string()
        };
        self.expect_symbol("{")?;

        let mut footprint = None;
        let mut zones = Vec::new();
        let mut rooms = Vec::new();
        let mut courtyards = Vec::new();
        let mut outdoor_areas = Vec::new();
        let mut objects = Vec::new();
        let mut openings = Vec::new();
        let mut wall_overrides = Vec::new();
        let mut assertions = Vec::new();
        let mut legend = None;
        let mut render = None;
        let mut dimensions = None;

        while !self.consume_symbol("}") {
            if self.check_keyword("footprint") {
                footprint = Some(self.parse_footprint()?);
            } else if self.check_keyword("zone") {
                zones.push(self.parse_zone()?);
            } else if self.check_keyword("courtyard") {
                courtyards.push(self.parse_courtyard()?);
            } else if self.check_keyword("outdoor") {
                outdoor_areas.push(self.parse_outdoor_area()?);
            } else if self.check_keyword("room") {
                rooms.push(self.parse_room()?);
            } else if self.check_keyword("object") {
                objects.push(self.parse_object()?);
            } else if self.check_keyword("opening") {
                openings.push(self.parse_opening()?);
            } else if self.check_keyword("wall_thickness") {
                wall_overrides.push(self.parse_wall_override()?);
            } else if self.check_keyword("assert") {
                assertions.push(self.parse_assertion()?);
            } else if self.check_keyword("legend") {
                legend = Some(self.parse_legend()?);
            } else if self.check_keyword("render") {
                render = Some(self.parse_render()?);
            } else if self.check_keyword("dimensions") {
                dimensions = Some(self.parse_dimensions()?);
            } else {
                return Err(self.error_here("Expected plan content"));
            }
        }

        let footprint = footprint.ok_or_else(|| self.error_here("Plan is missing footprint"))?;
        Ok(PlanDefinition {
            node_type: node_type("PlanDefinition"),
            name,
            footprint,
            zones,
            rooms,
            courtyards,
            outdoor_areas,
            objects,
            openings,
            wall_overrides,
            assertions,
            legend,
            render,
            dimensions,
        })
    }

    fn parse_dimensions(&mut self) -> Result<DimensionDeclaration, ParseError> {
        self.expect_keyword("dimensions")?;
        self.expect_symbol("{")?;
        let mut dimensions = DimensionDeclaration {
            node_type: node_type("DimensionDeclaration"),
            ..Default::default()
        };

        while !self.consume_symbol("}") {
            if self.consume_keyword("units") {
                dimensions.unit_system = Some(self.parse_dimension_unit_system()?);
            } else if self.consume_keyword("rooms") {
                dimensions.rooms = self.parse_dimension_room_selection()?;
            } else if self.consume_keyword("footprint") {
                dimensions.footprint = self.parse_dimension_bool()?;
            } else if self.consume_keyword("walls") || self.consume_keyword("wall") {
                dimensions.walls = self.parse_dimension_wall_selection()?;
            } else if self.consume_keyword("fixtures")
                || self.consume_keyword("fixture")
                || self.consume_keyword("objects")
                || self.consume_keyword("object")
            {
                dimensions.fixtures = self.parse_dimension_fixture_selection()?;
            } else {
                return Err(self.error_here("Expected dimensions content"));
            }
        }

        Ok(dimensions)
    }

    fn parse_dimension_units_declaration(
        &mut self,
    ) -> Result<DimensionUnitsDeclaration, ParseError> {
        self.expect_keyword("dimension_units")?;
        Ok(DimensionUnitsDeclaration {
            node_type: node_type("DimensionUnitsDeclaration"),
            units: self.parse_dimension_unit_system()?,
        })
    }

    fn parse_dimension_unit_system(&mut self) -> Result<DimensionUnitSystem, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "metric" => Ok(DimensionUnitSystem::Metric),
            "standard" | "imperial" | "us" | "us_customary" => Ok(DimensionUnitSystem::Standard),
            _ => Err(self.error_here(format!("Unknown dimension unit system '{raw}'"))),
        }
    }

    fn parse_dimension_room_selection(&mut self) -> Result<DimensionRoomSelection, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "all" | "on" | "true" => Ok(DimensionRoomSelection::All),
            "none" | "off" | "false" => Ok(DimensionRoomSelection::None),
            _ => Err(self.error_here(format!("Unknown room dimension mode '{raw}'"))),
        }
    }

    fn parse_dimension_bool(&mut self) -> Result<bool, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "all" | "on" | "true" => Ok(true),
            "none" | "off" | "false" => Ok(false),
            _ => Err(self.error_here(format!("Unknown dimension toggle '{raw}'"))),
        }
    }

    fn parse_dimension_wall_selection(&mut self) -> Result<DimensionWallSelection, ParseError> {
        if self.consume_keyword("all") {
            return Ok(DimensionWallSelection::All);
        }
        if self.consume_keyword("none") || self.consume_keyword("off") {
            return Ok(DimensionWallSelection::None);
        }

        let mut targets = Vec::new();
        while matches!(self.current().kind, TokenKind::Ident(_)) && !self.is_dimension_item_start()
        {
            let name = self.expect_ident()?;
            if self.consume_symbol(".") {
                targets.push(DimensionWallTarget::RoomEdge {
                    room: name,
                    edge: self.parse_edge_side()?,
                });
            } else {
                targets.push(DimensionWallTarget::WallId { id: name });
            }
        }

        if targets.is_empty() {
            Err(self.error_here("Expected wall dimension target"))
        } else {
            Ok(DimensionWallSelection::Only { targets })
        }
    }

    fn parse_dimension_fixture_selection(
        &mut self,
    ) -> Result<DimensionFixtureSelection, ParseError> {
        if self.consume_keyword("all") {
            return Ok(DimensionFixtureSelection::All);
        }
        if self.consume_keyword("none") || self.consume_keyword("off") {
            return Ok(DimensionFixtureSelection::None);
        }

        let mut names = Vec::new();
        while matches!(self.current().kind, TokenKind::Ident(_)) && !self.is_dimension_item_start()
        {
            names.push(self.expect_ident()?);
        }

        if names.is_empty() {
            Err(self.error_here("Expected fixture dimension target"))
        } else {
            Ok(DimensionFixtureSelection::Only { names })
        }
    }

    fn is_dimension_item_start(&self) -> bool {
        self.check_keyword("units")
            || self.check_keyword("rooms")
            || self.check_keyword("footprint")
            || self.check_keyword("walls")
            || self.check_keyword("wall")
            || self.check_keyword("fixtures")
            || self.check_keyword("fixture")
            || self.check_keyword("objects")
            || self.check_keyword("object")
    }

    fn parse_footprint(&mut self) -> Result<Footprint, ParseError> {
        self.expect_keyword("footprint")?;
        if self.consume_keyword("polygon") {
            Ok(Footprint::FootprintPolygon {
                points: self.parse_point_list()?,
            })
        } else if self.consume_keyword("rect") {
            Ok(Footprint::FootprintRect {
                p1: self.parse_point()?,
                p2: self.parse_point()?,
            })
        } else {
            Err(self.error_here("Expected footprint geometry"))
        }
    }

    fn parse_zone(&mut self) -> Result<ZoneDefinition, ParseError> {
        self.expect_keyword("zone")?;
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut label = None;
        let mut rooms = Vec::new();
        let mut attach = None;
        let mut align = None;
        let mut gap = None;
        while !self.consume_symbol("}") {
            if self.consume_keyword("label") {
                label = Some(self.expect_string()?);
            } else if self.check_keyword("room") {
                rooms.push(self.parse_room()?);
            } else if self.check_keyword("attach") {
                attach = Some(self.parse_attach()?);
            } else if self.check_keyword("align") {
                align = Some(self.parse_align()?);
            } else if self.check_keyword("gap") {
                gap = Some(self.parse_gap()?);
            } else {
                return Err(self.error_here("Expected zone content"));
            }
        }
        Ok(ZoneDefinition {
            node_type: node_type("ZoneDefinition"),
            name,
            label,
            rooms,
            attach,
            align,
            gap,
        })
    }

    fn parse_courtyard(&mut self) -> Result<CourtyardDefinition, ParseError> {
        self.expect_keyword("courtyard")?;
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut label = None;
        let mut geometry = None;
        while !self.consume_symbol("}") {
            if self.consume_keyword("label") {
                label = Some(self.expect_string()?);
            } else if self.consume_keyword("rect") {
                geometry = Some(CourtyardGeometry::CourtyardRect {
                    p1: self.parse_point()?,
                    p2: self.parse_point()?,
                });
            } else if self.consume_keyword("polygon") {
                geometry = Some(CourtyardGeometry::CourtyardPolygon {
                    points: self.parse_point_list()?,
                });
            } else {
                return Err(self.error_here("Expected courtyard content"));
            }
        }
        Ok(CourtyardDefinition {
            node_type: node_type("CourtyardDefinition"),
            name,
            label,
            geometry: geometry.ok_or_else(|| self.error_here("Courtyard is missing geometry"))?,
        })
    }

    fn parse_outdoor_area(&mut self) -> Result<OutdoorAreaDefinition, ParseError> {
        self.expect_keyword("outdoor")?;
        let raw_kind = self.expect_ident()?;
        let kind = OutdoorAreaKind::from_token(&raw_kind)
            .ok_or_else(|| self.error_here(format!("Unknown outdoor area kind '{raw_kind}'")))?;
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut label = None;
        let mut geometry = None;
        let mut floor = None;

        while !self.consume_symbol("}") {
            if self.consume_keyword("label") {
                label = Some(self.expect_string()?);
            } else if self.consume_keyword("floor") {
                floor = Some(self.parse_material_id()?);
            } else if self.consume_keyword("polygon") {
                geometry = Some(OutdoorGeometry::OutdoorPolygon {
                    points: self.parse_point_list()?,
                });
            } else if self.consume_keyword("rect") {
                geometry = Some(self.parse_outdoor_rect_geometry()?);
            } else {
                return Err(self.error_here("Expected outdoor area content"));
            }
        }

        Ok(OutdoorAreaDefinition {
            node_type: node_type("OutdoorAreaDefinition"),
            kind,
            name,
            label,
            geometry: geometry
                .ok_or_else(|| self.error_here("Outdoor area is missing geometry"))?,
            floor,
        })
    }

    fn parse_outdoor_rect_geometry(&mut self) -> Result<OutdoorGeometry, ParseError> {
        if self.consume_keyword("at") {
            let at = self.parse_point()?;
            self.expect_keyword("size")?;
            return Ok(OutdoorGeometry::OutdoorRectAtSize {
                at,
                size: self.parse_point()?,
            });
        }

        if self.consume_keyword("center") {
            let center = self.parse_point()?;
            self.expect_keyword("size")?;
            return Ok(OutdoorGeometry::OutdoorRectCenterSize {
                center,
                size: self.parse_point()?,
            });
        }

        Ok(OutdoorGeometry::OutdoorRect {
            p1: self.parse_point()?,
            p2: self.parse_point()?,
        })
    }

    fn parse_legend(&mut self) -> Result<LegendDeclaration, ParseError> {
        self.expect_keyword("legend")?;
        self.expect_symbol("{")?;
        let mut floor_materials = FloorMaterialLegendMode::Auto;
        while !self.consume_symbol("}") {
            if self.consume_keyword("floor_materials") {
                floor_materials = self.parse_floor_material_legend_mode()?;
            } else {
                return Err(self.error_here("Expected legend content"));
            }
        }
        Ok(LegendDeclaration {
            node_type: node_type("LegendDeclaration"),
            floor_materials,
        })
    }

    fn parse_floor_material_legend_mode(&mut self) -> Result<FloorMaterialLegendMode, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "auto" => Ok(FloorMaterialLegendMode::Auto),
            "show" => Ok(FloorMaterialLegendMode::Show),
            "hide" => Ok(FloorMaterialLegendMode::Hide),
            _ => Err(self.error_here(format!("Unknown floor material legend mode '{raw}'"))),
        }
    }

    fn parse_room(&mut self) -> Result<RoomDefinition, ParseError> {
        self.expect_keyword("room")?;
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut label = None;
        let mut geometry = None;
        let mut attach = None;
        let mut align = None;
        let mut gap = None;
        let mut extend = None;
        let mut floor = None;
        let mut fill_width = None;
        let mut fill_height = None;

        while !self.consume_symbol("}") {
            if self.consume_keyword("label") {
                label = Some(self.expect_string()?);
            } else if self.check_keyword("attach") {
                attach = Some(self.parse_attach()?);
            } else if self.check_keyword("align") {
                align = Some(self.parse_align()?);
            } else if self.check_keyword("gap") {
                gap = Some(self.parse_gap()?);
            } else if self.check_keyword("extend") {
                extend = Some(self.parse_extend()?);
            } else if self.consume_keyword("floor") {
                floor = Some(self.parse_material_id()?);
            } else if self.consume_keyword("width") {
                fill_width = Some(self.expect_number()?);
            } else if self.consume_keyword("height") {
                fill_height = Some(self.expect_number()?);
            } else {
                geometry = Some(self.parse_room_geometry()?);
            }
        }

        let mut geometry = geometry.ok_or_else(|| self.error_here("Room is missing geometry"))?;
        if let RoomGeometry::RoomFill { width, height, .. } = &mut geometry {
            *width = fill_width;
            *height = fill_height;
        }

        Ok(RoomDefinition {
            node_type: node_type("RoomDefinition"),
            name,
            label,
            geometry,
            floor,
            attach,
            align,
            gap,
            extend,
        })
    }

    fn parse_room_geometry(&mut self) -> Result<RoomGeometry, ParseError> {
        if self.consume_keyword("polygon") {
            return Ok(RoomGeometry::RoomPolygon {
                points: self.parse_point_list()?,
            });
        }

        if self.consume_keyword("fill") {
            self.expect_keyword("between")?;
            let room1 = self.expect_ident()?;
            self.expect_keyword("and")?;
            let room2 = self.expect_ident()?;
            return Ok(RoomGeometry::RoomFill {
                between: [room1, room2],
                width: None,
                height: None,
            });
        }

        self.expect_keyword("rect")?;
        if self.consume_keyword("span") {
            self.expect_keyword("x")?;
            self.expect_keyword("from")?;
            let from = self.parse_edge_reference()?;
            self.expect_keyword("to")?;
            let to = self.parse_edge_reference()?;
            self.expect_keyword("y")?;
            self.expect_symbol("(")?;
            let y1 = self.expect_number()?;
            self.expect_symbol(",")?;
            let y2 = self.expect_number()?;
            self.expect_symbol(")")?;
            return Ok(RoomGeometry::RoomRectSpan {
                span_x: SpanX {
                    node_type: node_type("SpanX"),
                    from,
                    to,
                },
                span_y: SpanY {
                    node_type: node_type("SpanY"),
                    from: y1,
                    to: y2,
                },
            });
        }

        if self.consume_keyword("at") {
            let at = self.parse_point()?;
            self.expect_keyword("size")?;
            return Ok(RoomGeometry::RoomRectAtSize {
                at,
                size: self.parse_point()?,
            });
        }

        if self.consume_keyword("center") {
            let center = self.parse_point()?;
            self.expect_keyword("size")?;
            return Ok(RoomGeometry::RoomRectCenterSize {
                center,
                size: self.parse_point()?,
            });
        }

        if self.consume_keyword("size") {
            return Ok(RoomGeometry::RoomRectSizeOnly {
                size: self.parse_size_value()?,
            });
        }

        Ok(RoomGeometry::RoomRectDiagonal {
            p1: self.parse_point()?,
            p2: self.parse_point()?,
        })
    }

    fn parse_size_value(&mut self) -> Result<SizeValue, ParseError> {
        self.expect_symbol("(")?;
        let x = self.parse_dimension_value()?;
        self.expect_symbol(",")?;
        let y = self.parse_dimension_value()?;
        self.expect_symbol(")")?;
        Ok(SizeValue { x, y })
    }

    fn parse_dimension_value(&mut self) -> Result<DimensionValue, ParseError> {
        if self.consume_keyword("auto") {
            Ok(DimensionValue::auto())
        } else {
            Ok(DimensionValue::Number(self.expect_number()?))
        }
    }

    fn parse_attach(&mut self) -> Result<AttachDirective, ParseError> {
        self.expect_keyword("attach")?;
        let raw = self.expect_ident()?.to_lowercase();
        let direction = match raw.as_str() {
            "north_of" => RelativeDirection::NorthOf,
            "south_of" => RelativeDirection::SouthOf,
            "east_of" => RelativeDirection::EastOf,
            "west_of" => RelativeDirection::WestOf,
            _ => return Err(self.error_here(format!("Unknown attach direction '{raw}'"))),
        };
        Ok(AttachDirective {
            node_type: node_type("AttachDirective"),
            direction,
            target: self.expect_ident()?,
        })
    }

    fn parse_align(&mut self) -> Result<AlignDirective, ParseError> {
        self.expect_keyword("align")?;
        if self.consume_keyword("my") {
            let my_edge = self.parse_align_edge()?;
            self.expect_keyword("with")?;
            let with_room = self.expect_ident()?;
            self.expect_symbol(".")?;
            let with_edge = self.parse_align_edge()?;
            Ok(AlignDirective::explicit(my_edge, with_room, with_edge))
        } else {
            Ok(AlignDirective::simple(self.parse_alignment_type()?))
        }
    }

    fn parse_alignment_type(&mut self) -> Result<AlignmentType, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "top" => Ok(AlignmentType::Top),
            "bottom" => Ok(AlignmentType::Bottom),
            "left" => Ok(AlignmentType::Left),
            "right" => Ok(AlignmentType::Right),
            "center" => Ok(AlignmentType::Center),
            _ => Err(self.error_here(format!("Unknown alignment '{raw}'"))),
        }
    }

    fn parse_align_edge(&mut self) -> Result<AlignEdge, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "top" => Ok(AlignEdge::Top),
            "bottom" => Ok(AlignEdge::Bottom),
            "left" => Ok(AlignEdge::Left),
            "right" => Ok(AlignEdge::Right),
            _ => Err(self.error_here(format!("Unknown alignment edge '{raw}'"))),
        }
    }

    fn parse_gap(&mut self) -> Result<GapDirective, ParseError> {
        self.expect_keyword("gap")?;
        Ok(GapDirective {
            node_type: node_type("GapDirective"),
            distance: self.expect_number()?,
        })
    }

    fn parse_extend(&mut self) -> Result<ExtendDirective, ParseError> {
        self.expect_keyword("extend")?;
        self.expect_keyword("from")?;
        let from = self.parse_edge_reference()?;
        self.expect_keyword("to")?;
        let to = self.parse_edge_reference()?;
        let axis = match from.edge {
            EdgeRefSide::Top | EdgeRefSide::Bottom => Axis::Y,
            EdgeRefSide::Left | EdgeRefSide::Right => Axis::X,
        };
        Ok(ExtendDirective {
            node_type: node_type("ExtendDirective"),
            axis,
            from,
            to,
        })
    }

    fn parse_edge_reference(&mut self) -> Result<EdgeReference, ParseError> {
        let room = self.expect_ident()?;
        self.expect_symbol(".")?;
        let edge = self.parse_edge_ref_side()?;
        Ok(EdgeReference { room, edge })
    }

    fn parse_edge_ref_side(&mut self) -> Result<EdgeRefSide, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "left" => Ok(EdgeRefSide::Left),
            "right" => Ok(EdgeRefSide::Right),
            "top" => Ok(EdgeRefSide::Top),
            "bottom" => Ok(EdgeRefSide::Bottom),
            _ => Err(self.error_here(format!("Unknown edge '{raw}'"))),
        }
    }

    fn parse_opening(&mut self) -> Result<Opening, ParseError> {
        self.expect_keyword("opening")?;
        if self.consume_keyword("double") {
            self.expect_keyword("door")?;
            self.parse_door_opening(true)
        } else if self.consume_keyword("double_door") {
            self.parse_door_opening(true)
        } else if self.consume_keyword("door") {
            self.parse_door_opening(false)
        } else if self.consume_keyword("window") {
            self.parse_window_opening()
        } else {
            Err(self.error_here("Expected opening type"))
        }
    }

    fn parse_door_opening(&mut self, default_double: bool) -> Result<Opening, ParseError> {
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut between = None;
        let mut room = None;
        let mut edge = None;
        let mut at = Position::Absolute { value: 0.0 };
        let mut width = None;
        let mut swing = None;
        let mut swing_room = None;
        let mut double = default_double;

        while !self.consume_symbol("}") {
            if self.consume_keyword("between") {
                let room1 = self.expect_ident()?;
                self.expect_keyword("and")?;
                let room2 = self.expect_ident()?;
                between = Some([room1, room2]);
            } else if self.consume_keyword("on") {
                if self.consume_keyword("shared_edge") {
                    // Marker only.
                } else {
                    let target_room = self.expect_ident()?;
                    self.expect_symbol(".")?;
                    self.expect_keyword("edge")?;
                    room = Some(target_room);
                    edge = Some(self.parse_edge_side()?);
                }
            } else if self.consume_keyword("at") {
                at = self.parse_position()?;
            } else if self.consume_keyword("width") {
                width = Some(self.expect_number()?);
            } else if self.consume_keyword("swing") {
                let raw = self.expect_ident()?;
                if let Some(door_swing) = DoorSwing::from_token(&raw) {
                    swing = Some(door_swing);
                } else {
                    swing_room = Some(raw);
                }
            } else if self.consume_keyword("double") {
                double = true;
            } else {
                return Err(self.error_here("Expected door content"));
            }
        }

        Ok(Opening::DoorOpening(DoorOpening {
            name,
            between,
            on: Some("shared_edge".to_string()).filter(|_| room.is_none()),
            room,
            edge,
            at,
            width,
            swing,
            swing_room,
            double,
        }))
    }

    fn parse_window_opening(&mut self) -> Result<Opening, ParseError> {
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut room = None;
        let mut edge = None;
        let mut at = Position::Absolute { value: 0.0 };
        let mut width = None;
        let mut sill = None;

        while !self.consume_symbol("}") {
            if self.consume_keyword("on") {
                room = Some(self.expect_ident()?);
                self.expect_symbol(".")?;
                self.expect_keyword("edge")?;
                edge = Some(self.parse_edge_side()?);
            } else if self.consume_keyword("at") {
                at = self.parse_position()?;
            } else if self.consume_keyword("width") {
                width = Some(self.expect_number()?);
            } else if self.consume_keyword("sill") {
                sill = Some(self.expect_number()?);
            } else {
                return Err(self.error_here("Expected window content"));
            }
        }

        Ok(Opening::WindowOpening(WindowOpening {
            name,
            room: room.ok_or_else(|| self.error_here("Window is missing room edge"))?,
            edge: edge.ok_or_else(|| self.error_here("Window is missing edge"))?,
            at,
            width,
            sill,
        }))
    }

    fn parse_position(&mut self) -> Result<Position, ParseError> {
        let value = self.expect_number()?;
        if self.consume_symbol("%") {
            Ok(Position::Percentage { value })
        } else {
            Ok(Position::Absolute { value })
        }
    }

    fn parse_edge_side(&mut self) -> Result<EdgeSide, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "north" => Ok(EdgeSide::North),
            "south" => Ok(EdgeSide::South),
            "east" => Ok(EdgeSide::East),
            "west" => Ok(EdgeSide::West),
            _ => Err(self.error_here(format!("Unknown edge side '{raw}'"))),
        }
    }

    fn parse_object(&mut self) -> Result<ObjectDefinition, ParseError> {
        self.expect_keyword("object")?;
        let name = self.expect_ident()?;
        self.expect_symbol("{")?;
        let mut catalog_id = None;
        let mut room = None;
        let mut label = None;
        let mut at = None;
        let mut attach = None;
        let mut facing = None;
        let mut rotate = None;
        let mut mirror = None;
        let mut clearance_overrides = Vec::new();

        while !self.consume_symbol("}") {
            if self.consume_keyword("use") {
                catalog_id = Some(self.parse_catalog_id()?);
            } else if self.consume_keyword("in") {
                room = Some(self.expect_ident()?);
            } else if self.consume_keyword("label") {
                label = Some(self.expect_string()?);
            } else if self.consume_keyword("at") {
                if self.check_symbol("(") {
                    at = Some(ObjectPosition::Point {
                        point: self.parse_point()?,
                    });
                } else {
                    at = Some(ObjectPosition::Distance {
                        position: self.parse_position()?,
                    });
                }
            } else if self.consume_keyword("attach") {
                let edge = self.parse_edge_side()?;
                self.expect_keyword("wall")?;
                attach = Some(ObjectWallAttachment {
                    node_type: node_type("ObjectWallAttachment"),
                    edge,
                });
            } else if self.consume_keyword("facing") {
                facing = Some(self.parse_edge_side()?);
            } else if self.consume_keyword("rotate") {
                rotate = Some(self.expect_number()?);
            } else if self.consume_keyword("mirror") {
                mirror = Some(self.parse_mirror_axis()?);
            } else if self.consume_keyword("clearance") {
                clearance_overrides.push(ObjectClearanceOverride {
                    node_type: node_type("ObjectClearanceOverride"),
                    side: self.parse_clearance_side()?,
                    value: self.expect_number()?,
                });
            } else {
                return Err(self.error_here("Expected object content"));
            }
        }

        Ok(ObjectDefinition {
            node_type: node_type("ObjectDefinition"),
            name,
            catalog_id: catalog_id.ok_or_else(|| self.error_here("Object is missing use"))?,
            room: room.ok_or_else(|| self.error_here("Object is missing room"))?,
            label,
            at,
            attach,
            facing,
            rotate,
            mirror,
            clearance_overrides,
        })
    }

    fn parse_catalog_id(&mut self) -> Result<String, ParseError> {
        let mut parts = vec![self.expect_ident()?];
        while self.consume_symbol(".") {
            parts.push(self.expect_ident()?);
        }
        Ok(parts.join("."))
    }

    fn parse_material_id(&mut self) -> Result<String, ParseError> {
        Ok(self.parse_catalog_id()?.to_ascii_lowercase())
    }

    fn parse_mirror_axis(&mut self) -> Result<MirrorAxis, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "x" => Ok(MirrorAxis::X),
            "y" => Ok(MirrorAxis::Y),
            _ => Err(self.error_here(format!("Unknown mirror axis '{raw}'"))),
        }
    }

    fn parse_clearance_side(&mut self) -> Result<ClearanceSide, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "front" => Ok(ClearanceSide::Front),
            "back" => Ok(ClearanceSide::Back),
            "left" => Ok(ClearanceSide::Left),
            "right" => Ok(ClearanceSide::Right),
            _ => Err(self.error_here(format!("Unknown clearance side '{raw}'"))),
        }
    }

    fn parse_wall_override(&mut self) -> Result<WallThicknessOverride, ParseError> {
        self.expect_keyword("wall_thickness")?;
        let room = self.expect_ident()?;
        self.expect_symbol(".")?;
        let edge = self.parse_edge_side()?;
        Ok(WallThicknessOverride {
            node_type: node_type("WallThicknessOverride"),
            room,
            edge,
            thickness: self.expect_number()?,
        })
    }

    fn parse_assertion(&mut self) -> Result<Assertion, ParseError> {
        self.expect_keyword("assert")?;
        if self.consume_keyword("inside") {
            self.expect_keyword("footprint")?;
            let target = self.expect_ident()?;
            return Ok(Assertion::AssertionInsideFootprint { target });
        }
        if self.consume_keyword("no_overlap") {
            self.expect_keyword("rooms")?;
            return Ok(Assertion::AssertionNoOverlap {
                target: "rooms".to_string(),
            });
        }
        if self.consume_keyword("openings_on_walls") {
            return Ok(Assertion::AssertionOpeningsOnWalls);
        }
        if self.consume_keyword("min_room_area") {
            let room = self.expect_ident()?;
            self.expect_symbol(">=")?;
            return Ok(Assertion::AssertionMinRoomArea {
                room,
                min_area: self.expect_number()?,
            });
        }
        if self.consume_keyword("rooms_connected") {
            return Ok(Assertion::AssertionRoomsConnected);
        }
        if self.consume_keyword("objects_inside_rooms") {
            return Ok(Assertion::AssertionObjectsInsideRooms);
        }
        if self.consume_keyword("object_no_overlap") {
            return Ok(Assertion::AssertionObjectNoOverlap);
        }
        if self.consume_keyword("object_clearances") {
            return Ok(Assertion::AssertionObjectClearances);
        }
        if self.consume_keyword("orientation") {
            let room = self.expect_ident()?;
            if self.consume_keyword("has_window") {
                return Ok(Assertion::AssertionOrientationHasWindow {
                    room,
                    target: self.parse_orientation_target()?,
                });
            }
            if self.consume_keyword("near") {
                self.expect_keyword("street")?;
                return Ok(Assertion::AssertionOrientationNearStreet { room });
            }
            if self.consume_keyword("away_from") {
                self.expect_keyword("street")?;
                return Ok(Assertion::AssertionOrientationAwayFromStreet { room });
            }
            if self.consume_keyword("garden_view") {
                return Ok(Assertion::AssertionOrientationGardenView { room });
            }
        }
        Err(self.error_here("Expected assertion"))
    }

    fn parse_orientation_target(&mut self) -> Result<OrientationTarget, ParseError> {
        let raw = self.expect_ident()?.to_lowercase();
        match raw.as_str() {
            "morning_sun" => Ok(OrientationTarget::MorningSun),
            "afternoon_sun" => Ok(OrientationTarget::AfternoonSun),
            "good_sun" => Ok(OrientationTarget::GoodSun),
            "street" => Ok(OrientationTarget::Street),
            "north" => Ok(OrientationTarget::North),
            "south" => Ok(OrientationTarget::South),
            "east" => Ok(OrientationTarget::East),
            "west" => Ok(OrientationTarget::West),
            _ => Err(self.error_here(format!("Unknown orientation target '{raw}'"))),
        }
    }

    fn parse_point(&mut self) -> Result<Point, ParseError> {
        self.expect_symbol("(")?;
        let x = self.expect_number()?;
        self.expect_symbol(",")?;
        let y = self.expect_number()?;
        self.expect_symbol(")")?;
        Ok(Point { x, y })
    }

    fn parse_point_list(&mut self) -> Result<Vec<Point>, ParseError> {
        let mut points = Vec::new();
        if self.consume_symbol("[") {
            while !self.consume_symbol("]") {
                points.push(self.parse_point()?);
                self.consume_symbol(",");
            }
        } else {
            points.push(self.parse_point()?);
            while self.check_symbol("(") {
                points.push(self.parse_point()?);
            }
        }
        Ok(points)
    }
}
