/* tslint:disable */
/* eslint-disable */
/**
 * Portobello API
 * Primary REST API for Portobello
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: sophie@sophiekatz.us
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {
  ConfigurationEntrySetResponse,
  ConfigurationKeySetResponse,
  ConfigurationTypeSetResponse,
  ErrorWithMessageResponse,
} from '../models';
import {
    ConfigurationEntrySetResponseFromJSON,
    ConfigurationEntrySetResponseToJSON,
    ConfigurationKeySetResponseFromJSON,
    ConfigurationKeySetResponseToJSON,
    ConfigurationTypeSetResponseFromJSON,
    ConfigurationTypeSetResponseToJSON,
    ErrorWithMessageResponseFromJSON,
    ErrorWithMessageResponseToJSON,
} from '../models';

/**
 * 
 */
export class DefaultApi extends runtime.BaseAPI {

    /**
     * Gets the current values of all configuration keys
     * List configuration values
     */
    async getConfigurationRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ConfigurationEntrySetResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/configuration`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ConfigurationEntrySetResponseFromJSON(jsonValue));
    }

    /**
     * Gets the current values of all configuration keys
     * List configuration values
     */
    async getConfiguration(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ConfigurationEntrySetResponse> {
        const response = await this.getConfigurationRaw(initOverrides);
        return await response.value();
    }

    /**
     * Returns a map of named configuration keys
     * List configuration keys
     */
    async getConfigurationKeysRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ConfigurationKeySetResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/configuration/keys`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ConfigurationKeySetResponseFromJSON(jsonValue));
    }

    /**
     * Returns a map of named configuration keys
     * List configuration keys
     */
    async getConfigurationKeys(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ConfigurationKeySetResponse> {
        const response = await this.getConfigurationKeysRaw(initOverrides);
        return await response.value();
    }

    /**
     * Returns a map of named types for configuration values
     * List types for configuration values
     */
    async getConfigurationTypesRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ConfigurationTypeSetResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/configuration/types`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ConfigurationTypeSetResponseFromJSON(jsonValue));
    }

    /**
     * Returns a map of named types for configuration values
     * List types for configuration values
     */
    async getConfigurationTypes(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ConfigurationTypeSetResponse> {
        const response = await this.getConfigurationTypesRaw(initOverrides);
        return await response.value();
    }

}