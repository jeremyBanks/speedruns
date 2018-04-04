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
    return body.data;
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
      ...gameCategories.map(category => {
        return new CategoryLevelPair({
          gameId: this.gameId,
          levelId: level.id,
          categoryId: category.id,
          categoryId: category.id,
        });
        return category.name;
      }),
      ...levels.map(level => levelCategories.map(category => {
        return new CategoryLevelPair({
          gameId: this.gameId,
          levelId: level.id,
          categoryId: category.id,
        });
      }))
    ];
  }
}

export class CategoryLevelPair {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.gameId =
    this.categoryId =
    this.levelId = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }

  async runs() {
    const runs = api(
      `runs?game=${this.gameId}&category=${this.categoryId
      }&status=verified&orderby=verify-date&direction=desc`);

    debugger;
  }
}

export class Run {
  constructor(...args) {
    this['ℹ️'] = this.constructor.name;
    this.runId =
    this.runner =
    this.duration =
    this.date = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
}
