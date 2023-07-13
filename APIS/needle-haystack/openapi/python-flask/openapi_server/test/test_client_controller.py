# coding: utf-8

from __future__ import absolute_import
import unittest

from flask import json
from six import BytesIO

from openapi_server.models.client import Client  # noqa: E501
from openapi_server.test import BaseTestCase


class TestClientController(BaseTestCase):
    """ClientController integration test stubs"""

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_create_client(self):
        """Test case for create_client

        Create Client
        """
        client = {"address":"address","active":False,"last_login_date":"01-01-1968","fk_group_id":0,"credit_rating":0.0,"fk_client_id":0,"phone":"phone","name":"46","created_date":"01-01-1968","id":102,"updated_date":"01-01-1968","email":"email","status":False}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/client',
            method='POST',
            headers=headers,
            data=json.dumps(client),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_create_clients_with_list_input(self):
        """Test case for create_clients_with_list_input

        Creates list of Client with given input array
        """
        client = {"address":"address","active":False,"last_login_date":"01-01-1968","fk_group_id":0,"credit_rating":0.0,"fk_client_id":0,"phone":"phone","name":"46","created_date":"01-01-1968","id":102,"updated_date":"01-01-1968","email":"email","status":False}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/client/createWithList',
            method='POST',
            headers=headers,
            data=json.dumps(client),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_delete_client(self):
        """Test case for delete_client

        Delete client
        """
        headers = { 
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/client/{name}'.format(name='name_example'),
            method='DELETE',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_get_client_by_name(self):
        """Test case for get_client_by_name

        Get client by name
        """
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/client/{name}'.format(name='name_example'),
            method='GET',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_update_client(self):
        """Test case for update_client

        Update client
        """
        client = {"address":"address","active":False,"last_login_date":"01-01-1968","fk_group_id":0,"credit_rating":0.0,"fk_client_id":0,"phone":"phone","name":"46","created_date":"01-01-1968","id":102,"updated_date":"01-01-1968","email":"email","status":False}
        headers = { 
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/client/{name}'.format(name='name_example'),
            method='PUT',
            headers=headers,
            data=json.dumps(client),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))


if __name__ == '__main__':
    unittest.main()
