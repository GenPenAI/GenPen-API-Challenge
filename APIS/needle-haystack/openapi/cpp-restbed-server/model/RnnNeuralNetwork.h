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

/*
 * RnnNeuralNetwork.h
 *
 * 
 */

#ifndef RnnNeuralNetwork_H_
#define RnnNeuralNetwork_H_



#include <string>
#include <memory>
#include <vector>
#include <boost/property_tree/ptree.hpp>
#include "helpers.h"

namespace org {
namespace openapitools {
namespace server {
namespace model {

/// <summary>
/// 
/// </summary>
class  RnnNeuralNetwork 
{
public:
    RnnNeuralNetwork() = default;
    explicit RnnNeuralNetwork(boost::property_tree::ptree const& pt);
    virtual ~RnnNeuralNetwork() = default;

    RnnNeuralNetwork(const RnnNeuralNetwork& other) = default; // copy constructor
    RnnNeuralNetwork(RnnNeuralNetwork&& other) noexcept = default; // move constructor

    RnnNeuralNetwork& operator=(const RnnNeuralNetwork& other) = default; // copy assignment
    RnnNeuralNetwork& operator=(RnnNeuralNetwork&& other) noexcept = default; // move assignment

    std::string toJsonString(bool prettyJson = false) const;
    void fromJsonString(std::string const& jsonString);
    boost::property_tree::ptree toPropertyTree() const;
    void fromPropertyTree(boost::property_tree::ptree const& pt);


    /////////////////////////////////////////////
    /// RnnNeuralNetwork members

    /// <summary>
    /// This attribute is a variable named:NeuronConnections
    /// </summary>
    int64_t getNeuronConnections() const;
    void setNeuronConnections(int64_t value);

    /// <summary>
    /// This attribute is a variable named:ComputationalPower
    /// </summary>
    double getComputationalPower() const;
    void setComputationalPower(double value);

    /// <summary>
    /// This attribute is a variable named:TrainsData
    /// </summary>
    bool isTrainsData() const;
    void setTrainsData(bool value);

    /// <summary>
    /// This attribute is a variable named:SymmetricStructure
    /// </summary>
    bool isSymmetricStructure() const;
    void setSymmetricStructure(bool value);

    /// <summary>
    /// This attribute is a variable named:ActivationFunction
    /// </summary>
    std::string getActivationFunction() const;
    void setActivationFunction(std::string value);

    /// <summary>
    /// This attribute is a variable named:OutputSequence
    /// </summary>
    bool isOutputSequence() const;
    void setOutputSequence(bool value);

    /// <summary>
    /// This attribute is a variable named:TimeseriesAnalysis
    /// </summary>
    bool isTimeseriesAnalysis() const;
    void setTimeseriesAnalysis(bool value);

    /// <summary>
    /// This attribute is a variable named:SequentialData
    /// </summary>
    std::string getSequentialData() const;
    void setSequentialData(std::string value);

    /// <summary>
    /// This attribute is a variable named:WeightVectorUpdates
    /// </summary>
    double getWeightVectorUpdates() const;
    void setWeightVectorUpdates(double value);

    /// <summary>
    /// 
    /// </summary>
    int64_t getGPOAcd() const;
    void setGPOAcd(int64_t value);

    /// <summary>
    /// This attribute is a variable named:AdaptiveLearning
    /// </summary>
    bool isAdaptiveLearning() const;
    void setAdaptiveLearning(bool value);

    /// <summary>
    /// 
    /// </summary>
    std::string getGPO473() const;
    void setGPO473(std::string value);

    /// <summary>
    /// This attribute is a variable named:NonlinearityFunction
    /// </summary>
    std::string getNonlinearityFunction() const;
    void setNonlinearityFunction(std::string value);

    /// <summary>
    /// This attribute is a variable named:OutputDimensionality
    /// </summary>
    int64_t getOutputDimensionality() const;
    void setOutputDimensionality(int64_t value);

    /// <summary>
    /// This attribute is a variable named:MemoryCells
    /// </summary>
    int64_t getMemoryCells() const;
    void setMemoryCells(int64_t value);

    /// <summary>
    /// This attribute is a variable named:MultipleInputs
    /// </summary>
    bool isMultipleInputs() const;
    void setMultipleInputs(bool value);

    /// <summary>
    /// This attribute is a variable named:Recurrent
    /// </summary>
    bool isRecurrent() const;
    void setRecurrent(bool value);

protected:
    int64_t m_NeuronConnections = 0L;
    double m_ComputationalPower = 0.0;
    bool m_TrainsData = false;
    bool m_SymmetricStructure = false;
    std::string m_ActivationFunction = "";
    bool m_OutputSequence = false;
    bool m_TimeseriesAnalysis = false;
    std::string m_SequentialData = "";
    double m_WeightVectorUpdates = 0.0;
    int64_t m_GPO_acd = 9482L;
    bool m_AdaptiveLearning = false;
    std::string m_GPO_473 = "9483";
    std::string m_NonlinearityFunction = "";
    int64_t m_OutputDimensionality = 0L;
    int64_t m_MemoryCells = 0L;
    bool m_MultipleInputs = false;
    bool m_Recurrent = false;
};

std::vector<RnnNeuralNetwork> createRnnNeuralNetworkVectorFromJsonString(const std::string& json);

template<>
inline boost::property_tree::ptree toPt<RnnNeuralNetwork>(const RnnNeuralNetwork& val) {
    return val.toPropertyTree();
}

template<>
inline RnnNeuralNetwork fromPt<RnnNeuralNetwork>(const boost::property_tree::ptree& pt) {
    RnnNeuralNetwork ret;
    ret.fromPropertyTree(pt);
    return ret;
}

}
}
}
}

#endif /* RnnNeuralNetwork_H_ */
