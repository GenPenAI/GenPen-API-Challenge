/**
 * The GamblerController file is a very simple one, which does not need to be changed manually,
 * unless there's a case where business logic routes the request to an entity which is not
 * the service.
 * The heavy lifting of the Controller item is done in Request.js - that is where request
 * parameters are extracted and sent to the service, and where response is handled.
 */

const Controller = require('./Controller');
const service = require('../services/GamblerService');
const createGambler = async (request, response) => {
  await Controller.handleRequest(request, response, service.createGambler);
};

const createGamblersWithListInput = async (request, response) => {
  await Controller.handleRequest(request, response, service.createGamblersWithListInput);
};

const deleteGambler = async (request, response) => {
  await Controller.handleRequest(request, response, service.deleteGambler);
};

const getGamblerByName = async (request, response) => {
  await Controller.handleRequest(request, response, service.getGamblerByName);
};

const updateGambler = async (request, response) => {
  await Controller.handleRequest(request, response, service.updateGambler);
};


module.exports = {
  createGambler,
  createGamblersWithListInput,
  deleteGambler,
  getGamblerByName,
  updateGambler,
};
