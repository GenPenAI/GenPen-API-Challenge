/* eslint-disable no-unused-vars */
const Service = require('./Service');

/**
* Create SportingBet
* This can only be done by the logged in user.
*
* sportingBet SportingBet Created sportingBet object (optional)
* returns SportingBet
* */
const createSportingBet = ({ sportingBet }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        sportingBet,
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
* Creates list of SportingBet with given input array
* Creates list of sportingBet with given input array
*
* sportingBet List  (optional)
* returns SportingBet
* */
const createSportingBetsWithListInput = ({ sportingBet }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        sportingBet,
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
* Delete sportingBet
* This can only be done by the logged in user.
*
* name String The sportingBet that needs to be deleted by name
* no response value expected for this operation
* */
const deleteSportingBet = ({ name }) => new Promise(
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
* Get sportingBet by name
* 
*
* name String The name that needs to be fetched. Use sportingBet1 for testing. 
* returns SportingBet
* */
const getSportingBetByName = ({ name }) => new Promise(
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
* Update sportingBet
* This can only be done by the logged in user.
*
* name String name of sportingBet that needs to be deleted
* sportingBet SportingBet Update an existent sportingBet in the system (optional)
* no response value expected for this operation
* */
const updateSportingBet = ({ name, sportingBet }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
        sportingBet,
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
  createSportingBet,
  createSportingBetsWithListInput,
  deleteSportingBet,
  getSportingBetByName,
  updateSportingBet,
};
