import connexion
import six
from typing import Dict
from typing import Tuple
from typing import Union

from openapi_server.models.client import Client  # noqa: E501
from openapi_server import util


def create_client(client=None):  # noqa: E501
    """Create Client

    This can only be done by the logged in user. # noqa: E501

    :param client: Created client object
    :type client: dict | bytes

    :rtype: Union[Client, Tuple[Client, int], Tuple[Client, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        client = Client.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_clients_with_list_input(client=None):  # noqa: E501
    """Creates list of Client with given input array

    Creates list of client with given input array # noqa: E501

    :param client: 
    :type client: list | bytes

    :rtype: Union[Client, Tuple[Client, int], Tuple[Client, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        client = [Client.from_dict(d) for d in connexion.request.get_json()]  # noqa: E501
    return 'do some magic!'


def delete_client(name):  # noqa: E501
    """Delete client

    This can only be done by the logged in user. # noqa: E501

    :param name: The client that needs to be deleted by name
    :type name: str

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_client_by_name(name):  # noqa: E501
    """Get client by name

     # noqa: E501

    :param name: The name that needs to be fetched. Use client1 for testing. 
    :type name: str

    :rtype: Union[Client, Tuple[Client, int], Tuple[Client, int, Dict[str, str]]
    """
    return 'do some magic!'


def update_client(name, client=None):  # noqa: E501
    """Update client

    This can only be done by the logged in user. # noqa: E501

    :param name: name of client that needs to be deleted
    :type name: str
    :param client: Update an existent client in the system
    :type client: dict | bytes

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        client = Client.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
