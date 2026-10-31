import { customInstance as uiCustomInstance, type CustomInstanceOptions } from "@eunomia/ui";

export {
  setApiGatewayBase,
  getApiGatewayBase,
  UNAUTHORIZED_EVENT,
  type RequestConfig,
  type CustomInstanceOptions,
  type ErrorType,
  type BodyType,
} from "@eunomia/ui";

/** `@eunomia/ui`'s client, declared here so orval can read the mutator's signature. */
export const customInstance = <T>(url: string, options?: CustomInstanceOptions): Promise<T> =>
  uiCustomInstance<T>(url, options);
