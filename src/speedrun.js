// A light wrapper for speedrun.com API functionality we're using.
// Subject to frequent change; not appropriate for general use.

import {compareAll, compareDefault, nProps} from '/assets/bester/utils.js';
import {extraRuns} from '/assets/data/runs.js';
import {fetch} from '/assets/bester/deps.js';

export const speedrunDotComApiRootUrl = '/https://www.speedrun.com/api/v1/';

export const api = async (path, maxPages = 6) => {
  if (!apiCache.has(path)) {
    const result = apiFetch(path, maxPages).then(null, error => {
      apiCache.delete(path);
      throw error;
    });
    apiCache.set(path, result);
    return await result;
  } else {
    return await apiCache.get(path);
  }
};

export const apiCache = new Map();

const apiFetch = async (path, maxPages = Infinity, pastPages = 0, offset = 0) => {
  if (pastPages >= maxPages) {
      throw new Error(`got too many results (more than ${maxPages} pages/${offset} items)`);
  }
  const url = speedrunDotComApiRootUrl + path + `&offset=${offset}`;
  const response = await fetch(url);
  const body = await response.json();3
  if (body.status) {
    throw new Error(`${body.status}: ${body.message}`); 
  } else {
    const {data} = body;
    if (body.pagination && body.pagination.links && body.pagination.links.filter(l => l.rel === 'next').length) {
      const rest = await apiFetch(path, maxPages, pastPages + 1, offset + body.pagination.max);
      return data.concat(rest);
    } else {
      return data;
    }
  }
};

export class Runner {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.isUser =
    this.userId =
    this.nick =
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
  
  static fromApiData(runner) {
    if (runner.rel === 'user') {
      return new Runner({
        isUser: true,
        userId: runner.id,
        nick: runner.names.international,
        url: runner.weblink,
      });
    } else {
      return new Runner({
        nick: runner.name,
        isUser: false
      });
    }
  }
}


export class Game {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.gameId =
    this.nick =
    this.slug =
    this.url =
    this.icon =
    this.categoryLevelPairs = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
 
  static async get(slug) {
    const data = await api(`games/${slug}?embed=categories,levels`);
    
    const levelCategories = data.categories.data.filter(c => c.type === 'per-level');
    const gameCategories = data.categories.data.filter(c => c.type === 'per-game');

    const categoryLevelPairs = [
      ...gameCategories.map(category => new CategoryLevelPair({
        gameId: this.gameId,
        levelId: null,
        categoryId: category.id,
        nick: `${category.name}`,
        url: category.weblink,
      })),
      ...[].concat(...data.levels.data.map(level => levelCategories.map(category => new CategoryLevelPair({
        gameId: this.gameId,
        levelId: level.id,
        categoryId: category.id,
        nick: `${level.name} (${category.name})`,
        url: level.weblink,
      }))))
    ];
    
    return new Game({
      gameId: data.id,
      nick: data.names.international,
      url: data.weblink,
      icon: data.assets.icon.uri,
      slug: data.abbreviation || data.id,
      categoryLevelPairs,
    });
  }

  async runsByCategoryLevelPairs() {
    const runsData = await api(
      `runs?game=${this.gameId}&status=verified&orderby=date&direction=asc&max=200&embed=players`);
    
    const runs = await Promise.all(runsData.map(Run.fromApiData));
    
    return new Map(await Promise.all(this.categoryLevelPairs.map(async pair => [
      pair,
      runs
        .filter(r => r.levelId === pair.levelId && r.categoryId === pair.categoryId)
        .concat(await Promise.all((nProps(extraRuns, this.gameId, pair.categoryId, pair.levelId) || []).map(Run.fromApiData)))
        .sort(compareAll(
          (r, s) => compareDefault(r.durationSeconds, s.durationSeconds),
          (r, s) => compareDefault(r.date, s.date),
          (r, s) => compareDefault(r.dateTimeSubmitted, s.dateTimeSubmitted),
        ))
    ])));
  }
}

export class CategoryLevelPair {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.gameId =
    this.categoryId =
    this.levelId = 
    this.nick = 
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }

  get slug() {
    return [this.categoryId, this.levelId].filter(Boolean).join('-');
  }
}


export class Run {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.runId =
    this.runner =
    this.durationSeconds =
    this.durationText =
    this.date = 
    this.dateTimeSubmitted = 
    this.levelId = 
    this.categoryId = 
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
  
  static normalizeDurationText(durationText) {
    const match = /^[A-Z]{2}(?:([0-9]{1,2})H)?(?:([0-9]{1,2})M)?(?:([0-9]{1,2})(?:\.([0-9]{1,3}))?S)?$/.exec(durationText);
    if (!match) {
      console.error(`failed to normalize duration: ${durationText}`);
      return durationText;
    }
    const [full, hours, minutes, seconds, miliseconds] = match;
    const pieces = [];
    if (hours) pieces.push(String(hours).padStart(2, '0'), 'H');
    if (minutes) pieces.push(String(hours).padStart(2, '0'), 'M');
    if (hours) pieces.push(String(hours).padStart(2, '0'), 'H');
  }
  
  static async fromApiData(data) {
    let runner;

    if (data.players.data.length === 1) {
      runner = Runner.fromApiData(data.players.data[0]);
    } else {
      runner = new Runner({
        nick: `${data.players.data.length} players`,
        isUser: false
      });
    }

    return new Run({
      runId: data.id,
      runner,
      durationSeconds: data.times.primary_t,
      durationText: Run.normalizeDurationText(data.times.primary),
      date: data.date,
      dateTimeSubmitted: data.submitted,
      levelId: data.level,
      categoryId: data.category,
      url: data.weblink,
    });
  }
}
