# coding: utf-8

from __future__ import absolute_import
import unittest

from flask import json
from six import BytesIO

from openapi_server.models.bullion import Bullion  # noqa: E501
from openapi_server.test import BaseTestCase


class TestBullionController(BaseTestCase):
    """BullionController integration test stubs"""

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_create_bullion(self):
        """Test case for create_bullion

        Create Bullion
        """
        bullion = {"year_issued":0,"notes":"notes","shape":"shape","purity":0.0,"weight":0.0,"date_of_sale":"01-01-1968","date_of_purchase":"01-01-1968","purity_percentage":0.0,"price_per_oz":0.8008281904610115,"metal_type":"metal_type","num_ingots":0,"name":"68","fk_owner_id":6027456,"gold_troy_oz":0.0,"fk_id":0,"id":79,"value":0.0,"status":"status"}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/bullion',
            method='POST',
            headers=headers,
            data=json.dumps(bullion),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_create_bullions_with_list_input(self):
        """Test case for create_bullions_with_list_input

        Creates list of Bullion with given input array
        """
        bullion = {"year_issued":0,"notes":"notes","shape":"shape","purity":0.0,"weight":0.0,"date_of_sale":"01-01-1968","date_of_purchase":"01-01-1968","purity_percentage":0.0,"price_per_oz":0.8008281904610115,"metal_type":"metal_type","num_ingots":0,"name":"68","fk_owner_id":6027456,"gold_troy_oz":0.0,"fk_id":0,"id":79,"value":0.0,"status":"status"}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/bullion/createWithList',
            method='POST',
            headers=headers,
            data=json.dumps(bullion),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_delete_bullion(self):
        """Test case for delete_bullion

        Delete bullion
        """
        headers = { 
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/bullion/{name}'.format(name='name_example'),
            method='DELETE',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_get_bullion_by_name(self):
        """Test case for get_bullion_by_name

        Get bullion by name
        """
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/bullion/{name}'.format(name='name_example'),
            method='GET',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_update_bullion(self):
        """Test case for update_bullion

        Update bullion
        """
        bullion = {"year_issued":0,"notes":"notes","shape":"shape","purity":0.0,"weight":0.0,"date_of_sale":"01-01-1968","date_of_purchase":"01-01-1968","purity_percentage":0.0,"price_per_oz":0.8008281904610115,"metal_type":"metal_type","num_ingots":0,"name":"68","fk_owner_id":6027456,"gold_troy_oz":0.0,"fk_id":0,"id":79,"value":0.0,"status":"status"}
        headers = { 
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/bullion/{name}'.format(name='name_example'),
            method='PUT',
            headers=headers,
            data=json.dumps(bullion),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))


if __name__ == '__main__':
    unittest.main()
