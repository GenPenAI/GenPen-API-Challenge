/**
 * API Inspector
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 */

(function(root, factory) {
  if (typeof define === 'function' && define.amd) {
    // AMD.
    define(['expect.js', process.cwd()+'/src/index'], factory);
  } else if (typeof module === 'object' && module.exports) {
    // CommonJS-like environments that support module.exports, like Node.
    factory(require('expect.js'), require(process.cwd()+'/src/index'));
  } else {
    // Browser globals (root is window)
    factory(root.expect, root.ApiInspector);
  }
}(this, function(expect, ApiInspector) {
  'use strict';

  var instance;

  beforeEach(function() {
    instance = new ApiInspector.CreditApi();
  });

  var getProperty = function(object, getter, property) {
    // Use getter method if present; otherwise, get the property directly.
    if (typeof object[getter] === 'function')
      return object[getter]();
    else
      return object[property];
  }

  var setProperty = function(object, setter, property, value) {
    // Use setter method if present; otherwise, set the property directly.
    if (typeof object[setter] === 'function')
      object[setter](value);
    else
      object[property] = value;
  }

  describe('CreditApi', function() {
    describe('createCredit', function() {
      it('should call createCredit successfully', function(done) {
        //uncomment below and update the code to test createCredit
        //instance.createCredit(function(error) {
        //  if (error) throw error;
        //expect().to.be();
        //});
        done();
      });
    });
    describe('createCreditsWithListInput', function() {
      it('should call createCreditsWithListInput successfully', function(done) {
        //uncomment below and update the code to test createCreditsWithListInput
        //instance.createCreditsWithListInput(function(error) {
        //  if (error) throw error;
        //expect().to.be();
        //});
        done();
      });
    });
    describe('deleteCredit', function() {
      it('should call deleteCredit successfully', function(done) {
        //uncomment below and update the code to test deleteCredit
        //instance.deleteCredit(function(error) {
        //  if (error) throw error;
        //expect().to.be();
        //});
        done();
      });
    });
    describe('getCreditByName', function() {
      it('should call getCreditByName successfully', function(done) {
        //uncomment below and update the code to test getCreditByName
        //instance.getCreditByName(function(error) {
        //  if (error) throw error;
        //expect().to.be();
        //});
        done();
      });
    });
    describe('updateCredit', function() {
      it('should call updateCredit successfully', function(done) {
        //uncomment below and update the code to test updateCredit
        //instance.updateCredit(function(error) {
        //  if (error) throw error;
        //expect().to.be();
        //});
        done();
      });
    });
  });

}));
