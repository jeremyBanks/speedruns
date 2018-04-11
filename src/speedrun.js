import {compareAll, compareDefault} from '/src/utils.js';
import {extraData} from '/src/api-patches.js';

export const speedrunDotComApiRootUrl = '/https://www.speedrun.com/api/v1/';

export const api = async path => {
  if (!apiCache.has(path)) {
    const result = apiFetch(path).then(null, error => { apiCache.delete(path); throw error; });
    apiCache.set(path, result);
    return await result;
  } else {
    return await apiCache.get(path);
  }
};

export const apiCache = new window.Map();

const apiFetch = async path => {
  const url = speedrunDotComApiRootUrl + path;
  const response = await window.fetch(url, {headers: new Headers({
    // we have our own caching, and use this header on both
    // request and response to disable the browser cache.
    // this may improve request parlallizability. maybe.
    'Cache-Control': 'no-store'
  })});
  const body = await response.json();
  if (body.status) {
    throw new Error(`${body.status}: ${body.message}`); 
  } else {
    if (body.pagination && body.pagination.size > body.pagination.max) {
      throw new Error(`found ${body.pagination.size} items matching request, exceeding our maximum of ${body.pagination.max}`);
    } else {
      const data = body.data;
      if (extraData[path]) {
        data.push(...extraData[path].filter(Boolean));
      }
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

  static async get(slug) {
    const runner = await api(`users/${slug}`);
    return new Runner({
      isUser: true,
      userId: runner.id,
      nick: runner.names.international,
      url: runner.weblink,
    });
  }
}


export class Game {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.gameId =
    this.nick =
    this.slug =
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
 
  static async get(slug) {
    const data = await api(`games/${slug}`);
    return new Game({
      gameId: data.id,
      nick: data.names.international,
      url: data.weblink,
      slug: data.abbreviation || data.id,
    });
  }

  async categoryLevelPairs() {
    const [categories, levels] = await Promise.all([
      api(`games/${this.gameId}/categories`),
      api(`games/${this.gameId}/levels`)
    ]);

    const levelCategories = categories.filter(c => c.type === 'per-level');
    const gameCategories = categories.filter(c => c.type === 'per-game');

    return [
      ...gameCategories.map(category => new CategoryLevelPair({
        gameId: this.gameId,
        levelId: null,
        categoryId: category.id,
        nick: `${category.name}`,
        url: category.weblink,
      })),
      ...[].concat(...levels.map(level => levelCategories.map(category => new CategoryLevelPair({
        gameId: this.gameId,
        levelId: level.id,
        categoryId: category.id,
        nick: `${level.name} (${category.name})`,
        url: level.weblink,
      }))))
    ];
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
    return `${this.categoryId}-${this.levelId}`;
  }
  
  async runs() {
    const runs = await api(
      `runs?game=${this.gameId}&category=${this.categoryId}&level=${this.levelId}&status=verified&orderby=date&direction=asc&max=200`);
    const results = [];
    for (const data of runs) {
      let runner;
      
      if (data.players.length === 1) {
        const playerData = data.players[0];
        if (playerData.rel === 'user') {
          runner = await Runner.get(playerData.id);
        } else {
          runner = new Runner({
            nick: playerData.name,
            isUser: false
          });
        }
      } else {
        runner = new Runner({
          nick: `${data.players.length} players`,
          isUser: false
        });
      }
      
      results.push(new Run({
        runId: data.id,
        runner,
        durationSeconds: data.times.primary_t,
        durationText: data.times.primary.slice(2).toLowerCase(),
        date: data.date,
        dateTimeSubmitted: data.submitted,
        url: data.weblink,
      }));
    }

    return results.sort(compareAll(
      (r, s) => compareDefault(r.durationSeconds, s.durationSeconds),
      (r, s) => compareDefault(r.date, s.date),
      (r, s) => compareDefault(r.dateTimeSubmitted, s.dateTimeSubmitted),
    ));
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
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
}
