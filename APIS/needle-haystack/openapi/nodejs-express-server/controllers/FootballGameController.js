/**
 * The FootballGameController file is a very simple one, which does not need to be changed manually,
 * unless there's a case where business logic routes the request to an entity which is not
 * the service.
 * The heavy lifting of the Controller item is done in Request.js - that is where request
 * parameters are extracted and sent to the service, and where response is handled.
 */

const Controller = require('./Controller');
const service = require('../services/FootballGameService');
const createFootballGame = async (request, response) => {
  await Controller.handleRequest(request, response, service.createFootballGame);
};

const createFootballGamesWithListInput = async (request, response) => {
  await Controller.handleRequest(request, response, service.createFootballGamesWithListInput);
};

const deleteFootballGame = async (request, response) => {
  await Controller.handleRequest(request, response, service.deleteFootballGame);
};

const getFootballGameByName = async (request, response) => {
  await Controller.handleRequest(request, response, service.getFootballGameByName);
};

const updateFootballGame = async (request, response) => {
  await Controller.handleRequest(request, response, service.updateFootballGame);
};


module.exports = {
  createFootballGame,
  createFootballGamesWithListInput,
  deleteFootballGame,
  getFootballGameByName,
  updateFootballGame,
};
