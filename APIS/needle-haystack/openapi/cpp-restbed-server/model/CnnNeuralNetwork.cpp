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



#include "CnnNeuralNetwork.h"

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

CnnNeuralNetwork::CnnNeuralNetwork(boost::property_tree::ptree const& pt)
{
        fromPropertyTree(pt);
}


std::string CnnNeuralNetwork::toJsonString(bool prettyJson /* = false */) const
{
	std::stringstream ss;
	write_json(ss, this->toPropertyTree(), prettyJson);
    // workaround inspired by: https://stackoverflow.com/a/56395440
    std::regex reg("\\\"([0-9]+\\.{0,1}[0-9]*)\\\"");
    std::string result = std::regex_replace(ss.str(), reg, "$1");
    return result;
}

void CnnNeuralNetwork::fromJsonString(std::string const& jsonString)
{
	std::stringstream ss(jsonString);
	ptree pt;
	read_json(ss,pt);
	this->fromPropertyTree(pt);
}

ptree CnnNeuralNetwork::toPropertyTree() const
{
	ptree pt;
	ptree tmp_node;
	pt.put("weights", m_Weights);
	pt.put("nodes", m_Nodes);
	pt.put("kernels", m_Kernels);
	pt.put("layers", m_Layers);
	pt.put("inputs", m_Inputs);
	pt.put("activation", m_Activation);
	pt.put("depth", m_Depth);
	pt.put("stride", m_Stride);
	pt.put("outputs", m_Outputs);
	pt.put("convolutionaloperation", m_Convolutionaloperation);
	pt.put("bias", m_Bias);
	pt.put("gPO-e33", m_GPO_e33);
	pt.put("pooling", m_Pooling);
	pt.put("gPO-916", m_GPO_916);
	pt.put("padding", m_Padding);
	pt.put("errorbackpropagation", m_Errorbackpropagation);
	pt.put("filters", m_Filters);
	return pt;
}

void CnnNeuralNetwork::fromPropertyTree(ptree const &pt)
{
	ptree tmp_node;
	m_Weights = pt.get("weights", 0.0);
	m_Nodes = pt.get("nodes", 0L);
	m_Kernels = pt.get("kernels", 0L);
	m_Layers = pt.get("layers", 0L);
	m_Inputs = pt.get("inputs", 0.0);
	m_Activation = pt.get("activation", "");
	m_Depth = pt.get("depth", 0L);
	m_Stride = pt.get("stride", 0L);
	m_Outputs = pt.get("outputs", 0.0);
	m_Convolutionaloperation = pt.get("convolutionaloperation", "");
	m_Bias = pt.get("bias", 0.0);
	m_GPO_e33 = pt.get("gPO-e33", 5860L);
	m_Pooling = pt.get("pooling", false);
	m_GPO_916 = pt.get("gPO-916", "5861");
	m_Padding = pt.get("padding", false);
	m_Errorbackpropagation = pt.get("errorbackpropagation", false);
	m_Filters = pt.get("filters", 0L);
}

double CnnNeuralNetwork::getWeights() const
{
    return m_Weights;
}

void CnnNeuralNetwork::setWeights(double value)
{
    m_Weights = value;
}


int64_t CnnNeuralNetwork::getNodes() const
{
    return m_Nodes;
}

void CnnNeuralNetwork::setNodes(int64_t value)
{
    m_Nodes = value;
}


int64_t CnnNeuralNetwork::getKernels() const
{
    return m_Kernels;
}

void CnnNeuralNetwork::setKernels(int64_t value)
{
    m_Kernels = value;
}


int64_t CnnNeuralNetwork::getLayers() const
{
    return m_Layers;
}

void CnnNeuralNetwork::setLayers(int64_t value)
{
    m_Layers = value;
}


double CnnNeuralNetwork::getInputs() const
{
    return m_Inputs;
}

void CnnNeuralNetwork::setInputs(double value)
{
    m_Inputs = value;
}


std::string CnnNeuralNetwork::getActivation() const
{
    return m_Activation;
}

void CnnNeuralNetwork::setActivation(std::string value)
{
    m_Activation = value;
}


int64_t CnnNeuralNetwork::getDepth() const
{
    return m_Depth;
}

void CnnNeuralNetwork::setDepth(int64_t value)
{
    m_Depth = value;
}


int64_t CnnNeuralNetwork::getStride() const
{
    return m_Stride;
}

void CnnNeuralNetwork::setStride(int64_t value)
{
    m_Stride = value;
}


double CnnNeuralNetwork::getOutputs() const
{
    return m_Outputs;
}

void CnnNeuralNetwork::setOutputs(double value)
{
    m_Outputs = value;
}


std::string CnnNeuralNetwork::getConvolutionaloperation() const
{
    return m_Convolutionaloperation;
}

void CnnNeuralNetwork::setConvolutionaloperation(std::string value)
{
    m_Convolutionaloperation = value;
}


double CnnNeuralNetwork::getBias() const
{
    return m_Bias;
}

void CnnNeuralNetwork::setBias(double value)
{
    m_Bias = value;
}


int64_t CnnNeuralNetwork::getGPOE33() const
{
    return m_GPO_e33;
}

void CnnNeuralNetwork::setGPOE33(int64_t value)
{
    m_GPO_e33 = value;
}


bool CnnNeuralNetwork::isPooling() const
{
    return m_Pooling;
}

void CnnNeuralNetwork::setPooling(bool value)
{
    m_Pooling = value;
}


std::string CnnNeuralNetwork::getGPO916() const
{
    return m_GPO_916;
}

void CnnNeuralNetwork::setGPO916(std::string value)
{
    m_GPO_916 = value;
}


bool CnnNeuralNetwork::isPadding() const
{
    return m_Padding;
}

void CnnNeuralNetwork::setPadding(bool value)
{
    m_Padding = value;
}


bool CnnNeuralNetwork::isErrorbackpropagation() const
{
    return m_Errorbackpropagation;
}

void CnnNeuralNetwork::setErrorbackpropagation(bool value)
{
    m_Errorbackpropagation = value;
}


int64_t CnnNeuralNetwork::getFilters() const
{
    return m_Filters;
}

void CnnNeuralNetwork::setFilters(int64_t value)
{
    m_Filters = value;
}



std::vector<CnnNeuralNetwork> createCnnNeuralNetworkVectorFromJsonString(const std::string& json)
{
    std::stringstream sstream(json);
    boost::property_tree::ptree pt;
    boost::property_tree::json_parser::read_json(sstream,pt);

    auto vec = std::vector<CnnNeuralNetwork>();
    for (const auto& child: pt) {
        vec.emplace_back(CnnNeuralNetwork(child.second));
    }

    return vec;
}

}
}
}
}

