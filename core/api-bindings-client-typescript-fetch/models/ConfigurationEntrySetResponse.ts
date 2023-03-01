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

import { exists, mapValues } from '../runtime';
import type { ConfigurationEntryResponse } from './ConfigurationEntryResponse';
import {
    ConfigurationEntryResponseFromJSON,
    ConfigurationEntryResponseFromJSONTyped,
    ConfigurationEntryResponseToJSON,
} from './ConfigurationEntryResponse';

/**
 * A set of configuration entries
 * @export
 * @interface ConfigurationEntrySetResponse
 */
export interface ConfigurationEntrySetResponse extends Array<ConfigurationEntryResponse> {
}

/**
 * Check if a given object implements the ConfigurationEntrySetResponse interface.
 */
export function instanceOfConfigurationEntrySetResponse(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function ConfigurationEntrySetResponseFromJSON(json: any): ConfigurationEntrySetResponse {
    return ConfigurationEntrySetResponseFromJSONTyped(json, false);
}

export function ConfigurationEntrySetResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ConfigurationEntrySetResponse {
    return json;
}

export function ConfigurationEntrySetResponseToJSON(value?: ConfigurationEntrySetResponse | null): any {
    return value;
}

