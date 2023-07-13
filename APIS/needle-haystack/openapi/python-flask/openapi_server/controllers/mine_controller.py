import connexion
import six
from typing import Dict
from typing import Tuple
from typing import Union

from openapi_server.models.mine import Mine  # noqa: E501
from openapi_server import util


def create_mine(mine=None):  # noqa: E501
    """Create Mine

    This can only be done by the logged in user. # noqa: E501

    :param mine: Created mine object
    :type mine: dict | bytes

    :rtype: Union[Mine, Tuple[Mine, int], Tuple[Mine, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        mine = Mine.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_mines_with_list_input(mine=None):  # noqa: E501
    """Creates list of Mine with given input array

    Creates list of mine with given input array # noqa: E501

    :param mine: 
    :type mine: list | bytes

    :rtype: Union[Mine, Tuple[Mine, int], Tuple[Mine, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        mine = [Mine.from_dict(d) for d in connexion.request.get_json()]  # noqa: E501
    return 'do some magic!'


def delete_mine(name):  # noqa: E501
    """Delete mine

    This can only be done by the logged in user. # noqa: E501

    :param name: The mine that needs to be deleted by name
    :type name: str

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_mine_by_name(name):  # noqa: E501
    """Get mine by name

     # noqa: E501

    :param name: The name that needs to be fetched. Use mine1 for testing. 
    :type name: str

    :rtype: Union[Mine, Tuple[Mine, int], Tuple[Mine, int, Dict[str, str]]
    """
    return 'do some magic!'


def update_mine(name, mine=None):  # noqa: E501
    """Update mine

    This can only be done by the logged in user. # noqa: E501

    :param name: name of mine that needs to be deleted
    :type name: str
    :param mine: Update an existent mine in the system
    :type mine: dict | bytes

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        mine = Mine.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
