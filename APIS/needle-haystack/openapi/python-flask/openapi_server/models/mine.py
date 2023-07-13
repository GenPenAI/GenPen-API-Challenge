# coding: utf-8

from __future__ import absolute_import
from datetime import date, datetime  # noqa: F401

from typing import List, Dict  # noqa: F401

from openapi_server.models.base_model_ import Model
from openapi_server import util


class Mine(Model):
    """NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).

    Do not edit the class manually.
    """

    def __init__(self, capacity_unit=None, capacity=None, power_usage=None, duration_its_been_active=None, active=None, fk_sender_id=None, risk_level=None, created_date=None, fk_owner_id=None, mined=None, length=None, id=None, date_mined=None, width=None, name=None, commodity_type=None, miner_id=None):  # noqa: E501
        """Mine - a model defined in OpenAPI

        :param capacity_unit: The capacity_unit of this Mine.  # noqa: E501
        :type capacity_unit: str
        :param capacity: The capacity of this Mine.  # noqa: E501
        :type capacity: int
        :param power_usage: The power_usage of this Mine.  # noqa: E501
        :type power_usage: float
        :param duration_its_been_active: The duration_its_been_active of this Mine.  # noqa: E501
        :type duration_its_been_active: int
        :param active: The active of this Mine.  # noqa: E501
        :type active: bool
        :param fk_sender_id: The fk_sender_id of this Mine.  # noqa: E501
        :type fk_sender_id: int
        :param risk_level: The risk_level of this Mine.  # noqa: E501
        :type risk_level: int
        :param created_date: The created_date of this Mine.  # noqa: E501
        :type created_date: str
        :param fk_owner_id: The fk_owner_id of this Mine.  # noqa: E501
        :type fk_owner_id: int
        :param mined: The mined of this Mine.  # noqa: E501
        :type mined: bool
        :param length: The length of this Mine.  # noqa: E501
        :type length: int
        :param id: The id of this Mine.  # noqa: E501
        :type id: int
        :param date_mined: The date_mined of this Mine.  # noqa: E501
        :type date_mined: str
        :param width: The width of this Mine.  # noqa: E501
        :type width: int
        :param name: The name of this Mine.  # noqa: E501
        :type name: str
        :param commodity_type: The commodity_type of this Mine.  # noqa: E501
        :type commodity_type: str
        :param miner_id: The miner_id of this Mine.  # noqa: E501
        :type miner_id: int
        """
        self.openapi_types = {
            'capacity_unit': str,
            'capacity': int,
            'power_usage': float,
            'duration_its_been_active': int,
            'active': bool,
            'fk_sender_id': int,
            'risk_level': int,
            'created_date': str,
            'fk_owner_id': int,
            'mined': bool,
            'length': int,
            'id': int,
            'date_mined': str,
            'width': int,
            'name': str,
            'commodity_type': str,
            'miner_id': int
        }

        self.attribute_map = {
            'capacity_unit': 'capacity_unit',
            'capacity': 'capacity',
            'power_usage': 'power_usage',
            'duration_its_been_active': 'duration_its_been_active',
            'active': 'active',
            'fk_sender_id': 'fk_sender_id',
            'risk_level': 'risk_level',
            'created_date': 'created_date',
            'fk_owner_id': 'fk_owner_id',
            'mined': 'mined',
            'length': 'length',
            'id': 'id',
            'date_mined': 'date_mined',
            'width': 'width',
            'name': 'name',
            'commodity_type': 'commodity_type',
            'miner_id': 'miner_id'
        }

        self._capacity_unit = capacity_unit
        self._capacity = capacity
        self._power_usage = power_usage
        self._duration_its_been_active = duration_its_been_active
        self._active = active
        self._fk_sender_id = fk_sender_id
        self._risk_level = risk_level
        self._created_date = created_date
        self._fk_owner_id = fk_owner_id
        self._mined = mined
        self._length = length
        self._id = id
        self._date_mined = date_mined
        self._width = width
        self._name = name
        self._commodity_type = commodity_type
        self._miner_id = miner_id

    @classmethod
    def from_dict(cls, dikt) -> 'Mine':
        """Returns the dict as a model

        :param dikt: A dict.
        :type: dict
        :return: The Mine of this Mine.  # noqa: E501
        :rtype: Mine
        """
        return util.deserialize_model(dikt, cls)

    @property
    def capacity_unit(self):
        """Gets the capacity_unit of this Mine.

        This attribute is a variable named capacity_unit  # noqa: E501

        :return: The capacity_unit of this Mine.
        :rtype: str
        """
        return self._capacity_unit

    @capacity_unit.setter
    def capacity_unit(self, capacity_unit):
        """Sets the capacity_unit of this Mine.

        This attribute is a variable named capacity_unit  # noqa: E501

        :param capacity_unit: The capacity_unit of this Mine.
        :type capacity_unit: str
        """

        self._capacity_unit = capacity_unit

    @property
    def capacity(self):
        """Gets the capacity of this Mine.

        This attribute is a variable named capacity  # noqa: E501

        :return: The capacity of this Mine.
        :rtype: int
        """
        return self._capacity

    @capacity.setter
    def capacity(self, capacity):
        """Sets the capacity of this Mine.

        This attribute is a variable named capacity  # noqa: E501

        :param capacity: The capacity of this Mine.
        :type capacity: int
        """
        if capacity is not None and capacity > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `capacity`, must be a value less than or equal to `10000000`")  # noqa: E501
        if capacity is not None and capacity < 0:  # noqa: E501
            raise ValueError("Invalid value for `capacity`, must be a value greater than or equal to `0`")  # noqa: E501

        self._capacity = capacity

    @property
    def power_usage(self):
        """Gets the power_usage of this Mine.

        This attribute is a variable named power_usage  # noqa: E501

        :return: The power_usage of this Mine.
        :rtype: float
        """
        return self._power_usage

    @power_usage.setter
    def power_usage(self, power_usage):
        """Sets the power_usage of this Mine.

        This attribute is a variable named power_usage  # noqa: E501

        :param power_usage: The power_usage of this Mine.
        :type power_usage: float
        """

        self._power_usage = power_usage

    @property
    def duration_its_been_active(self):
        """Gets the duration_its_been_active of this Mine.

        This attribute is a variable named duration_its_been_active  # noqa: E501

        :return: The duration_its_been_active of this Mine.
        :rtype: int
        """
        return self._duration_its_been_active

    @duration_its_been_active.setter
    def duration_its_been_active(self, duration_its_been_active):
        """Sets the duration_its_been_active of this Mine.

        This attribute is a variable named duration_its_been_active  # noqa: E501

        :param duration_its_been_active: The duration_its_been_active of this Mine.
        :type duration_its_been_active: int
        """
        if duration_its_been_active is not None and duration_its_been_active > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `duration_its_been_active`, must be a value less than or equal to `10000000`")  # noqa: E501
        if duration_its_been_active is not None and duration_its_been_active < 0:  # noqa: E501
            raise ValueError("Invalid value for `duration_its_been_active`, must be a value greater than or equal to `0`")  # noqa: E501

        self._duration_its_been_active = duration_its_been_active

    @property
    def active(self):
        """Gets the active of this Mine.

        This attribute is a variable named active  # noqa: E501

        :return: The active of this Mine.
        :rtype: bool
        """
        return self._active

    @active.setter
    def active(self, active):
        """Sets the active of this Mine.

        This attribute is a variable named active  # noqa: E501

        :param active: The active of this Mine.
        :type active: bool
        """

        self._active = active

    @property
    def fk_sender_id(self):
        """Gets the fk_sender_id of this Mine.

        This attribute is a variable named fk_sender_id  # noqa: E501

        :return: The fk_sender_id of this Mine.
        :rtype: int
        """
        return self._fk_sender_id

    @fk_sender_id.setter
    def fk_sender_id(self, fk_sender_id):
        """Sets the fk_sender_id of this Mine.

        This attribute is a variable named fk_sender_id  # noqa: E501

        :param fk_sender_id: The fk_sender_id of this Mine.
        :type fk_sender_id: int
        """
        if fk_sender_id is not None and fk_sender_id > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `fk_sender_id`, must be a value less than or equal to `10000000`")  # noqa: E501
        if fk_sender_id is not None and fk_sender_id < 0:  # noqa: E501
            raise ValueError("Invalid value for `fk_sender_id`, must be a value greater than or equal to `0`")  # noqa: E501

        self._fk_sender_id = fk_sender_id

    @property
    def risk_level(self):
        """Gets the risk_level of this Mine.

        This attribute is a variable named risk_level  # noqa: E501

        :return: The risk_level of this Mine.
        :rtype: int
        """
        return self._risk_level

    @risk_level.setter
    def risk_level(self, risk_level):
        """Sets the risk_level of this Mine.

        This attribute is a variable named risk_level  # noqa: E501

        :param risk_level: The risk_level of this Mine.
        :type risk_level: int
        """

        self._risk_level = risk_level

    @property
    def created_date(self):
        """Gets the created_date of this Mine.

        This attribute is a variable named created_date  # noqa: E501

        :return: The created_date of this Mine.
        :rtype: str
        """
        return self._created_date

    @created_date.setter
    def created_date(self, created_date):
        """Sets the created_date of this Mine.

        This attribute is a variable named created_date  # noqa: E501

        :param created_date: The created_date of this Mine.
        :type created_date: str
        """

        self._created_date = created_date

    @property
    def fk_owner_id(self):
        """Gets the fk_owner_id of this Mine.

        This attribute is a variable named fk_owner_id  # noqa: E501

        :return: The fk_owner_id of this Mine.
        :rtype: int
        """
        return self._fk_owner_id

    @fk_owner_id.setter
    def fk_owner_id(self, fk_owner_id):
        """Sets the fk_owner_id of this Mine.

        This attribute is a variable named fk_owner_id  # noqa: E501

        :param fk_owner_id: The fk_owner_id of this Mine.
        :type fk_owner_id: int
        """
        if fk_owner_id is not None and fk_owner_id > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `fk_owner_id`, must be a value less than or equal to `10000000`")  # noqa: E501
        if fk_owner_id is not None and fk_owner_id < 0:  # noqa: E501
            raise ValueError("Invalid value for `fk_owner_id`, must be a value greater than or equal to `0`")  # noqa: E501

        self._fk_owner_id = fk_owner_id

    @property
    def mined(self):
        """Gets the mined of this Mine.

        This attribute is a variable named mined  # noqa: E501

        :return: The mined of this Mine.
        :rtype: bool
        """
        return self._mined

    @mined.setter
    def mined(self, mined):
        """Sets the mined of this Mine.

        This attribute is a variable named mined  # noqa: E501

        :param mined: The mined of this Mine.
        :type mined: bool
        """

        self._mined = mined

    @property
    def length(self):
        """Gets the length of this Mine.

        This attribute is a variable named length  # noqa: E501

        :return: The length of this Mine.
        :rtype: int
        """
        return self._length

    @length.setter
    def length(self, length):
        """Sets the length of this Mine.

        This attribute is a variable named length  # noqa: E501

        :param length: The length of this Mine.
        :type length: int
        """
        if length is not None and length > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `length`, must be a value less than or equal to `10000000`")  # noqa: E501
        if length is not None and length < 0:  # noqa: E501
            raise ValueError("Invalid value for `length`, must be a value greater than or equal to `0`")  # noqa: E501

        self._length = length

    @property
    def id(self):
        """Gets the id of this Mine.

        This is a GenPen.AI specific attribute  # noqa: E501

        :return: The id of this Mine.
        :rtype: int
        """
        return self._id

    @id.setter
    def id(self, id):
        """Sets the id of this Mine.

        This is a GenPen.AI specific attribute  # noqa: E501

        :param id: The id of this Mine.
        :type id: int
        """
        if id is not None and id > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `id`, must be a value less than or equal to `10000000`")  # noqa: E501
        if id is not None and id < 0:  # noqa: E501
            raise ValueError("Invalid value for `id`, must be a value greater than or equal to `0`")  # noqa: E501

        self._id = id

    @property
    def date_mined(self):
        """Gets the date_mined of this Mine.

        This attribute is a variable named date_mined  # noqa: E501

        :return: The date_mined of this Mine.
        :rtype: str
        """
        return self._date_mined

    @date_mined.setter
    def date_mined(self, date_mined):
        """Sets the date_mined of this Mine.

        This attribute is a variable named date_mined  # noqa: E501

        :param date_mined: The date_mined of this Mine.
        :type date_mined: str
        """

        self._date_mined = date_mined

    @property
    def width(self):
        """Gets the width of this Mine.

        This attribute is a variable named width  # noqa: E501

        :return: The width of this Mine.
        :rtype: int
        """
        return self._width

    @width.setter
    def width(self, width):
        """Sets the width of this Mine.

        This attribute is a variable named width  # noqa: E501

        :param width: The width of this Mine.
        :type width: int
        """
        if width is not None and width > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `width`, must be a value less than or equal to `10000000`")  # noqa: E501
        if width is not None and width < 0:  # noqa: E501
            raise ValueError("Invalid value for `width`, must be a value greater than or equal to `0`")  # noqa: E501

        self._width = width

    @property
    def name(self):
        """Gets the name of this Mine.

        This is a GenPen.AI specific attribute  # noqa: E501

        :return: The name of this Mine.
        :rtype: str
        """
        return self._name

    @name.setter
    def name(self, name):
        """Sets the name of this Mine.

        This is a GenPen.AI specific attribute  # noqa: E501

        :param name: The name of this Mine.
        :type name: str
        """

        self._name = name

    @property
    def commodity_type(self):
        """Gets the commodity_type of this Mine.

        This attribute is a variable named commodity_type  # noqa: E501

        :return: The commodity_type of this Mine.
        :rtype: str
        """
        return self._commodity_type

    @commodity_type.setter
    def commodity_type(self, commodity_type):
        """Sets the commodity_type of this Mine.

        This attribute is a variable named commodity_type  # noqa: E501

        :param commodity_type: The commodity_type of this Mine.
        :type commodity_type: str
        """

        self._commodity_type = commodity_type

    @property
    def miner_id(self):
        """Gets the miner_id of this Mine.

        This attribute is a variable named miner_id  # noqa: E501

        :return: The miner_id of this Mine.
        :rtype: int
        """
        return self._miner_id

    @miner_id.setter
    def miner_id(self, miner_id):
        """Sets the miner_id of this Mine.

        This attribute is a variable named miner_id  # noqa: E501

        :param miner_id: The miner_id of this Mine.
        :type miner_id: int
        """
        if miner_id is not None and miner_id > 10000000:  # noqa: E501
            raise ValueError("Invalid value for `miner_id`, must be a value less than or equal to `10000000`")  # noqa: E501
        if miner_id is not None and miner_id < 0:  # noqa: E501
            raise ValueError("Invalid value for `miner_id`, must be a value greater than or equal to `0`")  # noqa: E501

        self._miner_id = miner_id