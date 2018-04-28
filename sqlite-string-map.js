// Based on https://gist.github.com/samliu/ab6c22e91205066281e57aa93408df05
// Original Author: Sam Liu (sam@ambushnetworks.com)
// License: MIT

import Sequelize from 'sequelize';

import {LazySymbolScope} from './src/bester/utils.js';


const internal = new LazySymbolScope('internal ');
const {
  DB,
  TABLE,
  INIT,
} = internal;


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
    
    this[TABLE] = this[INIT]();
  }
  
  async [INIT]() {
    await this[DB].authenticate();
    const tableName = this.constructor.name || 'kv';
    return this[DB].define(tableName, {
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
  }
  
  async clear() {
    const table = await this[TABLE];
    return await table.sync({force: true});
  }
  
  async get(key) {
    key = String(key);
    const table = await this[TABLE];
    const results = await table.findAll({
      where: {
        k: key
      }
    })
    
    console.log(results);
    return results[0].dataValues.v;
  }

  async set(key, value) {
    return await this.upsert(key, value);
  }

  async upsert(key, value) {
    key = String(key);
    value = String(value);
    const table = await this[TABLE];
    return await table.upsert({k: key, v: value});
  }
}
