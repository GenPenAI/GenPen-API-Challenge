/* eslint-disable no-unused-vars */
const Service = require('./Service');

/**
* Create FootballGame
* This can only be done by the logged in user.
*
* footballGame FootballGame Created footballGame object (optional)
* returns FootballGame
* */
const createFootballGame = ({ footballGame }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        footballGame,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Creates list of FootballGame with given input array
* Creates list of footballGame with given input array
*
* footballGame List  (optional)
* returns FootballGame
* */
const createFootballGamesWithListInput = ({ footballGame }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        footballGame,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Delete footballGame
* This can only be done by the logged in user.
*
* name String The footballGame that needs to be deleted by name
* no response value expected for this operation
* */
const deleteFootballGame = ({ name }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get footballGame by name
* 
*
* name String The name that needs to be fetched. Use footballGame1 for testing. 
* returns FootballGame
* */
const getFootballGameByName = ({ name }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Update footballGame
* This can only be done by the logged in user.
*
* name String name of footballGame that needs to be deleted
* footballGame FootballGame Update an existent footballGame in the system (optional)
* no response value expected for this operation
* */
const updateFootballGame = ({ name, footballGame }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
        footballGame,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);

module.exports = {
  createFootballGame,
  createFootballGamesWithListInput,
  deleteFootballGame,
  getFootballGameByName,
  updateFootballGame,
};
