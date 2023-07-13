/**
 * The SportingBetController file is a very simple one, which does not need to be changed manually,
 * unless there's a case where business logic routes the request to an entity which is not
 * the service.
 * The heavy lifting of the Controller item is done in Request.js - that is where request
 * parameters are extracted and sent to the service, and where response is handled.
 */

const Controller = require('./Controller');
const service = require('../services/SportingBetService');
const createSportingBet = async (request, response) => {
  await Controller.handleRequest(request, response, service.createSportingBet);
};

const createSportingBetsWithListInput = async (request, response) => {
  await Controller.handleRequest(request, response, service.createSportingBetsWithListInput);
};

const deleteSportingBet = async (request, response) => {
  await Controller.handleRequest(request, response, service.deleteSportingBet);
};

const getSportingBetByName = async (request, response) => {
  await Controller.handleRequest(request, response, service.getSportingBetByName);
};

const updateSportingBet = async (request, response) => {
  await Controller.handleRequest(request, response, service.updateSportingBet);
};


module.exports = {
  createSportingBet,
  createSportingBetsWithListInput,
  deleteSportingBet,
  getSportingBetByName,
  updateSportingBet,
};
