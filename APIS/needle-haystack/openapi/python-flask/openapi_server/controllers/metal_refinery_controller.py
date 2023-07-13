import connexion
import six
from typing import Dict
from typing import Tuple
from typing import Union

from openapi_server.models.metal_refinery import MetalRefinery  # noqa: E501
from openapi_server import util


def create_metal_refinery(metal_refinery=None):  # noqa: E501
    """Create MetalRefinery

    This can only be done by the logged in user. # noqa: E501

    :param metal_refinery: Created metalRefinery object
    :type metal_refinery: dict | bytes

    :rtype: Union[MetalRefinery, Tuple[MetalRefinery, int], Tuple[MetalRefinery, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        metal_refinery = MetalRefinery.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'


def create_metal_refinerys_with_list_input(metal_refinery=None):  # noqa: E501
    """Creates list of MetalRefinery with given input array

    Creates list of metalRefinery with given input array # noqa: E501

    :param metal_refinery: 
    :type metal_refinery: list | bytes

    :rtype: Union[MetalRefinery, Tuple[MetalRefinery, int], Tuple[MetalRefinery, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        metal_refinery = [MetalRefinery.from_dict(d) for d in connexion.request.get_json()]  # noqa: E501
    return 'do some magic!'


def delete_metal_refinery(name):  # noqa: E501
    """Delete metalRefinery

    This can only be done by the logged in user. # noqa: E501

    :param name: The metalRefinery that needs to be deleted by name
    :type name: str

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    return 'do some magic!'


def get_metal_refinery_by_name(name):  # noqa: E501
    """Get metalRefinery by name

     # noqa: E501

    :param name: The name that needs to be fetched. Use metalRefinery1 for testing. 
    :type name: str

    :rtype: Union[MetalRefinery, Tuple[MetalRefinery, int], Tuple[MetalRefinery, int, Dict[str, str]]
    """
    return 'do some magic!'


def update_metal_refinery(name, metal_refinery=None):  # noqa: E501
    """Update metalRefinery

    This can only be done by the logged in user. # noqa: E501

    :param name: name of metalRefinery that needs to be deleted
    :type name: str
    :param metal_refinery: Update an existent metalRefinery in the system
    :type metal_refinery: dict | bytes

    :rtype: Union[None, Tuple[None, int], Tuple[None, int, Dict[str, str]]
    """
    if connexion.request.is_json:
        metal_refinery = MetalRefinery.from_dict(connexion.request.get_json())  # noqa: E501
    return 'do some magic!'
