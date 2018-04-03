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
    this.id =
    this.nick =
    this.isUser =
    this.url = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }

  static async get(slug) {
    const runner = await api(`users/${slug}`);
    return new Runner({
      id: runner.id,
      nick: runner.names.international,
      url: runner.weblink,
    });
  }
}


export class Game {
  constructor(...args) {
    this.id =
    this.nick =
    this.categoryId =
    this.levelId = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }

  async categoryLevelPairs() {
    
  }
}

export class CategoryLevel {
  async runs() {

  }
}

export class Run {
  constructor(...args) {
    this.id =
    this.runner =
    this.duration = void this;
    Object.seal(this);
    Object.assign(this, ...args);
  }
}




