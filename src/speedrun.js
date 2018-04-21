// A light wrapper for speedrun.com API functionality we're using.
// Subject to frequent change; not appropriate for general use.

import {compareAll, compareDefault, nProps} from '/assets/bester/utils.js';
import {extraRuns} from '/assets/data/runs.js';
import {fetch} from '/assets/bester/deps.js';

export const speedrunDotComApiRootUrl = '/https://www.speedrun.com/api/v1/';

export const api = async (path, maxPages = 6) => {
  if (!apiCache.has(path)) {
    const result = apiFetch(path).then(null, error => {
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

const apiFetch = async path => {
  const url = speedrunDotComApiRootUrl + path;
  const response = await fetch(url);
  const body = await response.json();
  if (body.status) {
    throw new Error(`${body.status}: ${body.message}`); 
  } else {
    if (body.pagination && body.pagination.links && body.pagination.links.filter(l => l.rel === 'next').length) {
      throw new Error(`got too many results (more than one page (${body.pagination.max}))`);
    } else {
      const {data} = body;
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
      durationText: data.times.primary.slice(2).toLowerCase(),
      date: data.date,
      dateTimeSubmitted: data.submitted,
      levelId: data.level,
      categoryId: data.category,
      url: data.weblink,
    });
  }
}
