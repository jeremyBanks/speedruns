// Based on https://gist.github.com/samliu/ab6c22e91205066281e57aa93408df05
// Original Author: Sam Liu (sam@ambushnetworks.com)
// License: MIT

import Sequelize from 'sequelize';

import {LazySymbolScope} from './src/bester/utils.js';


const internal = new LazySymbolScope('internal ');
const {
  db,
  table,
  init,
} = internal;

export class SqliteStringMap {
  constructor(iterable, name) {
    const input = new Map(iterable);
    name = String(name);

    this[db] = new Sequelize(
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
    
    this.table = this[init]();
  }
  
  async init() {
    await this[db].authenticate();
    const tableName = this.constructor.name || 'kv';
    t = this[db].define(tableName, {
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
}

 
// Populate data store with default data (a single value containing )
module.exports.resetdb = function() {
  KVStore.sync({force: true}) // using 'force' it drops the table kvstore if it already exists, and creates a new one
    .then(function(){});  
}
 
// Gets all key-value pairs in the database and returns them in a list via callback.
module.exports.getAll = function(callback) {
  KVStore.findAll().then(function(kvpairs) { // find all entries in the kvstore table
    var dbContents=[];
    kvpairs.forEach(function(kvpair) {
      dbContents.push([kvpair.k,kvpair.v]); // adds their info to the dbContents value
    });
    callback(dbContents);
  });
}
 
// Retrieves a single value given a key and returns it via callback.
module.exports.get = function(key, callback) {
  KVStore.findAll({
    where: {
      k: key
    }
  }).then(function(results){
    var queryResult;
    results.forEach(function(result){
      if (result.dataValues.v) {
        queryResult = result.dataValues.v;
      }
    });
    if (queryResult) {
      callback(queryResult);
    } else {
      // If no response was sent (this query failed) just send a 404 not found.
      callback(false);
    }
  });
}
 
// Returns a promise to either update or insert the key to a new value.
module.exports.insert = function(key, value) {
  return KVStore.upsert({ k: key, v: value});
}