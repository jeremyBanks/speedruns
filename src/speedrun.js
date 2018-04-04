export const speedrunDotComApiRootUrl = '/https://www.speedrun.com/api/v1/';

export const api = async path => {
  if (!apiCache.has(path)) {
    const result = await apiFetch(path);
    apiCache.set(path, result);
    return result;
  } else {
    return apiCache.get(path);
  }
};

export const apiCache = new window.Map();

const apiFetch = async path => {
  const url = speedrunDotComApiRootUrl + path;
  const response = await window.fetch(url);
  const body = await response.json();
  if (body.status) {
    throw new Error(`${body.status}: ${body.message}`); 
  } else {
    if (body.pagination && body.pagination.size > body.pagination.max) {
      throw new Error(`found ${body.pagination.size} items matching request, exceeding our maximum of ${body.pagination.max}`);
    } else {
      return body.data;
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
        name: `${category.name}`,
      })),
      ...[].concat(...levels.map(level => levelCategories.map(category => new CategoryLevelPair({
        gameId: this.gameId,
        levelId: level.id,
        categoryId: category.id,
        name: `${level.name} (${category.name})`,
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
    this.name = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }

  async runs() {
    const runs = await api(
      `runs?game=${this.gameId}&category=${this.categoryId
      }&status=verified&orderby=date&direction=asc&max=200`);
    return runs.map(data => {
      let runner;
      
      if (data.players.length === 1) {
        const playerData = data.players[0];
        if (playerData.rel === 'user') {
          runner = Runner.get(playerData.id);
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
      
      return new Run({
        runId: data.id,
        runner,
        durationSeconds: data.times.primary_t,
        durationText: data.times.primary.slice(2).toLowerCase(),
        date: data.date,
        url: data.weblink,
      });
    }).sort((r, s) => r.durationSeconds - s.durationSeconds);
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
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
}
