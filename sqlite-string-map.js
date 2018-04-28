// Based on https://gist.github.com/samliu/ab6c22e91205066281e57aa93408df05
// Original Author: Sam Liu (sam@ambushnetworks.com)
// License: MIT

import Sequelize from 'sequelize';

import {LazySymbolScope} from './src/bester/utils.js';


const internal = new LazySymbolScope('internal ');
const {
  DB,
  CACHE,
  TABLE,
  INIT,
} = internal;



const assertString = s => {
  if (typeof s !== 'string') {
    throw new TypeError(`expected string, got ${typeof s} ${s}`);
  }
  return s;
};



// okay god what does this need to be?
// a two-tier cache, with promises committed in-memory, but never put in the database unless they resolve successfully.
export class SqliteStringMap {
  constructor(name) {
    name = String(name);

    this[DB] = new Sequelize(
      name,
      process.env.DB_USER,
      process.env.DB_PASS, {
        host: '0.0.0.0',
        dialect: 'sqlite',
        pool: {
          max: 5,
          min: 0,
          idle: 10000
        },
        // Security note: the database is saved to the file `database.sqlite` on the local filesystem. It's deliberately placed in the `.data` directory
        // which doesn't get copied if someone remixes the project.
        storage: '.data/database.sqlite'
      }
    );
    
    this[CACHE] = new Map();
    this[TABLE] = this[INIT]();
  }
  
  async [INIT]() {
    await this[DB].authenticate();
    const tableName = this.constructor.name || 'kv';
    const table = this[DB].define(tableName, {
      k: {
        type: Sequelize.STRING,
        allowNull: false,
        unique: true
      },
      v: {
        type: Sequelize.STRING,
        allowNull: false
      }
    });
    await table.sync();
    return table;
  }
  
  async clear() {
    const table = await this[TABLE];
    return await table.sync({force: true});
  }
  
  async get(key) {
    key = assertString(key);

    const cached = this[CACHE].get(key);
    if (cached !== undefined) {
      return cached;
    }
    
    const table = await this[TABLE];
    const results = await table.findAll({
      [Sequelize.Op.where]: {
        k: key
      }
    })
    
    if (results.length === 0) {
      return undefined;
    } else {
      return results[0].dataValues.v;
    }
  }

  async set(key, value) {
    key = assertString(key);
    value = Promise.resolve(value).then(assertString);

    this[CACHE].set(key, value);
    
    try {
      // if value is async, we want to resolve before committing to database
      const syncValue = await value;
      const table = await this[TABLE];
      return await table.upsert({k: key, v: value});
    } catch (ex) {
      // if promise or commit fails, also remove from in-memory cache:
      if (this[CACHE].get(key) === value) {
        this[CACHE].delete(key);
      }
    }
  }
}
