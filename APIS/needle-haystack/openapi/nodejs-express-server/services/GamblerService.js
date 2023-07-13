/* eslint-disable no-unused-vars */
const Service = require('./Service');

/**
* Create Gambler
* This can only be done by the logged in user.
*
* gambler Gambler Created gambler object (optional)
* returns Gambler
* */
const createGambler = ({ gambler }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        gambler,
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
* Creates list of Gambler with given input array
* Creates list of gambler with given input array
*
* gambler List  (optional)
* returns Gambler
* */
const createGamblersWithListInput = ({ gambler }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        gambler,
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
* Delete gambler
* This can only be done by the logged in user.
*
* name String The gambler that needs to be deleted by name
* no response value expected for this operation
* */
const deleteGambler = ({ name }) => new Promise(
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
* Get gambler by name
* 
*
* name String The name that needs to be fetched. Use gambler1 for testing. 
* returns Gambler
* */
const getGamblerByName = ({ name }) => new Promise(
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
* Update gambler
* This can only be done by the logged in user.
*
* name String name of gambler that needs to be deleted
* gambler Gambler Update an existent gambler in the system (optional)
* no response value expected for this operation
* */
const updateGambler = ({ name, gambler }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
        gambler,
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
  createGambler,
  createGamblersWithListInput,
  deleteGambler,
  getGamblerByName,
  updateGambler,
};
