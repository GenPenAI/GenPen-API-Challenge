# coding: utf-8

from __future__ import absolute_import
import unittest

from flask import json
from six import BytesIO

from openapi_server.models.metal_refinery import MetalRefinery  # noqa: E501
from openapi_server.test import BaseTestCase


class TestMetalRefineryController(BaseTestCase):
    """MetalRefineryController integration test stubs"""

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_create_metal_refinery(self):
        """Test case for create_metal_refinery

        Create MetalRefinery
        """
        metal_refinery = {"efficiency":0.8008281904610115,"cost":0.0,"life_expectancy":0,"open_time":"01-01-1968","fk_production_site_id":0,"close_time":"01-01-1968","type":"type","capacity":0,"fk_object_id":0,"name":"52","location":"location","id":60,"status":False}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/metalRefinery',
            method='POST',
            headers=headers,
            data=json.dumps(metal_refinery),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_create_metal_refinerys_with_list_input(self):
        """Test case for create_metal_refinerys_with_list_input

        Creates list of MetalRefinery with given input array
        """
        metal_refinery = {"efficiency":0.8008281904610115,"cost":0.0,"life_expectancy":0,"open_time":"01-01-1968","fk_production_site_id":0,"close_time":"01-01-1968","type":"type","capacity":0,"fk_object_id":0,"name":"52","location":"location","id":60,"status":False}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/metalRefinery/createWithList',
            method='POST',
            headers=headers,
            data=json.dumps(metal_refinery),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_delete_metal_refinery(self):
        """Test case for delete_metal_refinery

        Delete metalRefinery
        """
        headers = { 
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/metalRefinery/{name}'.format(name='name_example'),
            method='DELETE',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_get_metal_refinery_by_name(self):
        """Test case for get_metal_refinery_by_name

        Get metalRefinery by name
        """
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/metalRefinery/{name}'.format(name='name_example'),
            method='GET',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_update_metal_refinery(self):
        """Test case for update_metal_refinery

        Update metalRefinery
        """
        metal_refinery = {"efficiency":0.8008281904610115,"cost":0.0,"life_expectancy":0,"open_time":"01-01-1968","fk_production_site_id":0,"close_time":"01-01-1968","type":"type","capacity":0,"fk_object_id":0,"name":"52","location":"location","id":60,"status":False}
        headers = { 
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/metalRefinery/{name}'.format(name='name_example'),
            method='PUT',
            headers=headers,
            data=json.dumps(metal_refinery),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))


if __name__ == '__main__':
    unittest.main()
