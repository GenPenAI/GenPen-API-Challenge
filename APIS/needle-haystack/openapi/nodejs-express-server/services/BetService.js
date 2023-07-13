/* eslint-disable no-unused-vars */
const Service = require('./Service');

/**
* Create Bet
* This can only be done by the logged in user.
*
* bet Bet Created bet object (optional)
* returns Bet
* */
const createBet = ({ bet }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        bet,
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
* Creates list of Bet with given input array
* Creates list of bet with given input array
*
* bet List  (optional)
* returns Bet
* */
const createBetsWithListInput = ({ bet }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        bet,
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
* Delete bet
* This can only be done by the logged in user.
*
* name String The bet that needs to be deleted by name
* no response value expected for this operation
* */
const deleteBet = ({ name }) => new Promise(
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
* Get bet by name
* 
*
* name String The name that needs to be fetched. Use bet1 for testing. 
* returns Bet
* */
const getBetByName = ({ name }) => new Promise(
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
* Update bet
* This can only be done by the logged in user.
*
* name String name of bet that needs to be deleted
* bet Bet Update an existent bet in the system (optional)
* no response value expected for this operation
* */
const updateBet = ({ name, bet }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        name,
        bet,
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
  createBet,
  createBetsWithListInput,
  deleteBet,
  getBetByName,
  updateBet,
};
