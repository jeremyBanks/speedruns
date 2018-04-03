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

export class Player {
  constructor() {
    self.id = void String;
    self.nick = void String;
    self.url = void String;
  }

  static async get(slug) {
    const player = await api(`users/${slug}`);
    return Object.assign(new Player, {
      id: player.id,
      nick: player.names.international,
      url: player.weblink,
    });
  }
}


export class Game {
  async categoryLevelPairs() {
    
  }
}

export class CategoryLevel {
  async runs() {

  }
}

export class Run {
  
  static async get(slug) {
    const player = await api(`users/${slug}`);
    return Object.assign(new Player, {
      id: player.id,
      nick: player.names.international,
      url: player.weblink,
    });
  }
}




