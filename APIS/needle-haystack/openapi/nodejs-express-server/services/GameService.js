/* eslint-disable no-unused-vars */
const Service = require('./Service');

/**
* Create Game
* This can only be done by the logged in user.
*
* game Game Created game object (optional)
* returns Game
* */
const createGame = ({ game }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        game,
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
* Creates list of Game with given input array
* Creates list of game with given input array
*
* game List  (optional)
* returns Game
* */
const createGamesWithListInput = ({ game }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        game,
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
* Delete game
* This can only be done by the logged in user.
*
* name String The game that needs to be deleted by name
* no response value expected for this operation
* */
const deleteGame = ({ name }) => new Promise(
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
* Get game by name
* 
*
* name String The name that needs to be fetched. Use game1 for testing. 
* returns Game
* */
const getGameByName = ({ name }) => new Promise(
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
* Update game
* This can only be done by the logged in user.
*
* name String name of game that needs to be deleted
* game Game Update an existent game in the system (optional)
* no response value expected for this operation
* */
const updateGame = ({ name, game }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
        game,
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
  createGame,
  createGamesWithListInput,
  deleteGame,
  getGameByName,
  updateGame,
};
