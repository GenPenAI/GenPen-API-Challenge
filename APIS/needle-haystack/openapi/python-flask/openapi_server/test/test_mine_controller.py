# coding: utf-8

from __future__ import absolute_import
import unittest

from flask import json
from six import BytesIO

from openapi_server.models.mine import Mine  # noqa: E501
from openapi_server.test import BaseTestCase


class TestMineController(BaseTestCase):
    """MineController integration test stubs"""

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_create_mine(self):
        """Test case for create_mine

        Create Mine
        """
        mine = {"mined":False,"fk_sender_id":6027456,"capacity_unit":"capacity_unit","length":0,"active":False,"date_mined":"01-01-1968","commodity_type":"commodity_type","capacity":0,"risk_level":0,"power_usage":0.8008281904610115,"miner_id":5962133,"width":0,"name":"10","fk_owner_id":1465812,"duration_its_been_active":0,"created_date":"01-01-1968","id":119}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/mine',
            method='POST',
            headers=headers,
            data=json.dumps(mine),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_create_mines_with_list_input(self):
        """Test case for create_mines_with_list_input

        Creates list of Mine with given input array
        """
        mine = {"mined":False,"fk_sender_id":6027456,"capacity_unit":"capacity_unit","length":0,"active":False,"date_mined":"01-01-1968","commodity_type":"commodity_type","capacity":0,"risk_level":0,"power_usage":0.8008281904610115,"miner_id":5962133,"width":0,"name":"10","fk_owner_id":1465812,"duration_its_been_active":0,"created_date":"01-01-1968","id":119}
        headers = { 
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/mine/createWithList',
            method='POST',
            headers=headers,
            data=json.dumps(mine),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_delete_mine(self):
        """Test case for delete_mine

        Delete mine
        """
        headers = { 
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/mine/{name}'.format(name='name_example'),
            method='DELETE',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    def test_get_mine_by_name(self):
        """Test case for get_mine_by_name

        Get mine by name
        """
        headers = { 
            'Accept': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/mine/{name}'.format(name='name_example'),
            method='GET',
            headers=headers)
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))

    @unittest.skip("Connexion does not support multiple consumes. See https://github.com/zalando/connexion/pull/760")
    def test_update_mine(self):
        """Test case for update_mine

        Update mine
        """
        mine = {"mined":False,"fk_sender_id":6027456,"capacity_unit":"capacity_unit","length":0,"active":False,"date_mined":"01-01-1968","commodity_type":"commodity_type","capacity":0,"risk_level":0,"power_usage":0.8008281904610115,"miner_id":5962133,"width":0,"name":"10","fk_owner_id":1465812,"duration_its_been_active":0,"created_date":"01-01-1968","id":119}
        headers = { 
            'Content-Type': 'application/json',
        }
        response = self.client.open(
            '/openapi-jaxrs-server-1.0.0/api/v3/mine/{name}'.format(name='name_example'),
            method='PUT',
            headers=headers,
            data=json.dumps(mine),
            content_type='application/json')
        self.assert200(response,
                       'Response body is : ' + response.data.decode('utf-8'))


if __name__ == '__main__':
    unittest.main()
