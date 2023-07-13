import connexion
import six
from typing import Dict
from typing import Tuple
from typing import Union

from openapi_server.models.bullion import Bullion  # noqa: E501
from openapi_server import util


def create_bullion(bullion=None):  # noqa: E501
    """Create Bullion

    This can only be done by the logged in user. # noqa: E501

    :param bullion: Created bullion object
    :type bullion: dict | bytes

    :rtype: Union[Bullion, Tuple[Bullion, int], Tuple[Bullion, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        bullion = Bullion.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_bullions_with_list_input(bullion=None):  # noqa: E501
    """Creates list of Bullion with given input array

    Creates list of bullion with given input array # noqa: E501

    :param bullion: 
    :type bullion: list | bytes

    :rtype: Union[Bullion, Tuple[Bullion, int], Tuple[Bullion, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        bullion = [Bullion.from_dict(d) for d in connexion.request.get_json()]  # noqa: E501
    return 'do some magic!'


def delete_bullion(name):  # noqa: E501
    """Delete bullion

    This can only be done by the logged in user. # noqa: E501

    :param name: The bullion that needs to be deleted by name
    :type name: str

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_bullion_by_name(name):  # noqa: E501
    """Get bullion by name

     # noqa: E501

    :param name: The name that needs to be fetched. Use bullion1 for testing. 
    :type name: str

    :rtype: Union[Bullion, Tuple[Bullion, int], Tuple[Bullion, int, Dict[str, str]]
    """
    return 'do some magic!'


def update_bullion(name, bullion=None):  # noqa: E501
    """Update bullion

    This can only be done by the logged in user. # noqa: E501

    :param name: name of bullion that needs to be deleted
    :type name: str
    :param bullion: Update an existent bullion in the system
    :type bullion: dict | bytes

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        bullion = Bullion.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
