/**
 * API Inspector
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI-Generator 6.2.1.
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */



#include "Bullion.h"

#include <string>
#include <vector>
#include <map>
#include <sstream>
#include <stdexcept>
#include <regex>
#include <boost/lexical_cast.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>
#include "helpers.h"

using boost::property_tree::ptree;
using boost::property_tree::read_json;
using boost::property_tree::write_json;

namespace org {
namespace openapitools {
namespace server {
namespace model {

Bullion::Bullion(boost::property_tree::ptree const& pt)
{
        fromPropertyTree(pt);
}


std::string Bullion::toJsonString(bool prettyJson /* = false */) const
{
	std::stringstream ss;
	write_json(ss, this->toPropertyTree(), prettyJson);
    // workaround inspired by: https://stackoverflow.com/a/56395440
    std::regex reg("\\\"([0-9]+\\.{0,1}[0-9]*)\\\"");
    std::string result = std::regex_replace(ss.str(), reg, "$1");
    return result;
}

void Bullion::fromJsonString(std::string const& jsonString)
{
	std::stringstream ss(jsonString);
	ptree pt;
	read_json(ss,pt);
	this->fromPropertyTree(pt);
}

ptree Bullion::toPropertyTree() const
{
	ptree pt;
	ptree tmp_node;
	pt.put("status", m_Status);
	pt.put("shape", m_Shape);
	pt.put("price_per_oz", m_Price_per_oz);
	pt.put("purity_percentage", m_Purity_percentage);
	pt.put("notes", m_Notes);
	pt.put("value", m_Value);
	pt.put("year_issued", m_Year_issued);
	pt.put("purity", m_Purity);
	pt.put("date_of_sale", m_Date_of_sale);
	pt.put("gold_troy_oz", m_Gold_troy_oz);
	pt.put("fk_owner_id", m_Fk_owner_id);
	pt.put("num_ingots", m_Num_ingots);
	pt.put("id", m_Id);
	pt.put("weight", m_Weight);
	pt.put("date_of_purchase", m_Date_of_purchase);
	pt.put("name", m_Name);
	pt.put("fk_id", m_Fk_id);
	pt.put("metal_type", m_Metal_type);
	return pt;
}

void Bullion::fromPropertyTree(ptree const &pt)
{
	ptree tmp_node;
	m_Status = pt.get("status", "");
	m_Shape = pt.get("shape", "");
	m_Price_per_oz = pt.get("price_per_oz", 0.0);
	m_Purity_percentage = pt.get("purity_percentage", 0.0);
	m_Notes = pt.get("notes", "");
	m_Value = pt.get("value", 0.0);
	m_Year_issued = pt.get("year_issued", 0);
	m_Purity = pt.get("purity", 0.0);
	m_Date_of_sale = pt.get("date_of_sale", "");
	m_Gold_troy_oz = pt.get("gold_troy_oz", 0.0);
	m_Fk_owner_id = pt.get("fk_owner_id", 0);
	m_Num_ingots = pt.get("num_ingots", 0);
	m_Id = pt.get("id", 0);
	m_Weight = pt.get("weight", 0.0);
	m_Date_of_purchase = pt.get("date_of_purchase", "");
	m_Name = pt.get("name", "");
	m_Fk_id = pt.get("fk_id", 0);
	m_Metal_type = pt.get("metal_type", "");
}

std::string Bullion::getStatus() const
{
    return m_Status;
}

void Bullion::setStatus(std::string value)
{
    m_Status = value;
}


std::string Bullion::getShape() const
{
    return m_Shape;
}

void Bullion::setShape(std::string value)
{
    m_Shape = value;
}


double Bullion::getPricePerOz() const
{
    return m_Price_per_oz;
}

void Bullion::setPricePerOz(double value)
{
    m_Price_per_oz = value;
}


double Bullion::getPurityPercentage() const
{
    return m_Purity_percentage;
}

void Bullion::setPurityPercentage(double value)
{
    m_Purity_percentage = value;
}


std::string Bullion::getNotes() const
{
    return m_Notes;
}

void Bullion::setNotes(std::string value)
{
    m_Notes = value;
}


double Bullion::getValue() const
{
    return m_Value;
}

void Bullion::setValue(double value)
{
    m_Value = value;
}


int32_t Bullion::getYearIssued() const
{
    return m_Year_issued;
}

void Bullion::setYearIssued(int32_t value)
{
    m_Year_issued = value;
}


double Bullion::getPurity() const
{
    return m_Purity;
}

void Bullion::setPurity(double value)
{
    m_Purity = value;
}


std::string Bullion::getDateOfSale() const
{
    return m_Date_of_sale;
}

void Bullion::setDateOfSale(std::string value)
{
    m_Date_of_sale = value;
}


double Bullion::getGoldTroyOz() const
{
    return m_Gold_troy_oz;
}

void Bullion::setGoldTroyOz(double value)
{
    m_Gold_troy_oz = value;
}


int32_t Bullion::getFkOwnerId() const
{
    return m_Fk_owner_id;
}

void Bullion::setFkOwnerId(int32_t value)
{
    m_Fk_owner_id = value;
}


int32_t Bullion::getNumIngots() const
{
    return m_Num_ingots;
}

void Bullion::setNumIngots(int32_t value)
{
    m_Num_ingots = value;
}


int32_t Bullion::getId() const
{
    return m_Id;
}

void Bullion::setId(int32_t value)
{
    m_Id = value;
}


double Bullion::getWeight() const
{
    return m_Weight;
}

void Bullion::setWeight(double value)
{
    m_Weight = value;
}


std::string Bullion::getDateOfPurchase() const
{
    return m_Date_of_purchase;
}

void Bullion::setDateOfPurchase(std::string value)
{
    m_Date_of_purchase = value;
}


std::string Bullion::getName() const
{
    return m_Name;
}

void Bullion::setName(std::string value)
{
    m_Name = value;
}


int32_t Bullion::getFkId() const
{
    return m_Fk_id;
}

void Bullion::setFkId(int32_t value)
{
    m_Fk_id = value;
}


std::string Bullion::getMetalType() const
{
    return m_Metal_type;
}

void Bullion::setMetalType(std::string value)
{
    m_Metal_type = value;
}



std::vector<Bullion> createBullionVectorFromJsonString(const std::string& json)
{
    std::stringstream sstream(json);
    boost::property_tree::ptree pt;
    boost::property_tree::json_parser::read_json(sstream,pt);

    auto vec = std::vector<Bullion>();
    for (const auto& child: pt) {
        vec.emplace_back(Bullion(child.second));
    }

    return vec;
}

}
}
}
}

