import { describe, expect, it } from 'vitest';
import { readdirSync, readFileSync } from 'fs';
import { join } from 'path';
import { compile } from '../src/compiler.js';

const EXAMPLES_DIR = join(__dirname, '../examples');

function exampleFiles(): string[] {
  return readdirSync(EXAMPLES_DIR)
    .filter((file) => file.endsWith('.psc'))
    .sort();
}

function stripNonCode(source: string): string {
  return source
    .replace(/\/\*[\s\S]*?\*\//g, ' ')
    .replace(/\/\/[^\n]*/g, ' ')
    .replace(/#[^\n]*/g, ' ')
    .replace(/"[^"]*"/g, ' ');
}

function hasToken(corpus: string, token: string): boolean {
  const escaped = token.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  return new RegExp(`(^|[^a-z0-9_])${escaped}($|[^a-z0-9_])`, 'i').test(corpus);
}

describe('PlanScript examples', () => {
  it('compiles every .psc example', () => {
    const failures: string[] = [];

    for (const file of exampleFiles()) {
      const source = readFileSync(join(EXAMPLES_DIR, file), 'utf-8');
      const result = compile(source, { emitSVG: false });

      if (!result.success) {
        const messages = result.errors.map((error) => `${error.phase}: ${error.message}`).join('; ');
        failures.push(`${file}: ${messages}`);
      }
    }

    expect(failures).toEqual([]);
  });

  it('covers every parser keyword in the examples corpus', () => {
    const rawCorpus = exampleFiles()
      .map((file) => readFileSync(join(EXAMPLES_DIR, file), 'utf-8'))
      .join('\n');
    const codeCorpus = stripNonCode(rawCorpus).toLowerCase();

    const keywords = [
      'units',
      'meters',
      'm',
      'cm',
      'mm',
      'ft',
      'in',
      'origin',
      'axis',
      'x',
      'y',
      'right',
      'left',
      'up',
      'down',
      'grid',
      'defaults',
      'door_width',
      'window_width',
      'site',
      'street',
      'hemisphere',
      'north',
      'south',
      'east',
      'west',
      'plan',
      'footprint',
      'polygon',
      'rect',
      'zone',
      'label',
      'courtyard',
      'room',
      'at',
      'size',
      'center',
      'auto',
      'fill',
      'between',
      'and',
      'span',
      'from',
      'to',
      'attach',
      'north_of',
      'south_of',
      'east_of',
      'west_of',
      'align',
      'my',
      'with',
      'top',
      'bottom',
      'gap',
      'extend',
      'width',
      'height',
      'opening',
      'door',
      'window',
      'on',
      'edge',
      'shared_edge',
      'sill',
      'swing',
      'wall_thickness',
      'assert',
      'inside',
      'all_rooms',
      'no_overlap',
      'rooms',
      'openings_on_walls',
      'min_room_area',
      'rooms_connected',
      'orientation',
      'has_window',
      'morning_sun',
      'afternoon_sun',
      'good_sun',
      'near',
      'away_from',
      'garden_view',
    ];

    const missing = keywords.filter((keyword) => !hasToken(codeCorpus, keyword));

    expect(missing).toEqual([]);
    expect(rawCorpus).toContain('#');
    expect(rawCorpus).toContain('//');
    expect(rawCorpus).toContain('/*');
    expect(rawCorpus).toContain('*/');
    expect(codeCorpus).toContain('>=');
    expect(codeCorpus).toMatch(/\d+\s*%/);
  });
});
