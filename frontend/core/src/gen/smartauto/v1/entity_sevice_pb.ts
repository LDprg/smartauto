// @generated by protoc-gen-es v2.2.3 with parameter "target=ts"
// @generated from file smartauto/v1/entity_sevice.proto (package smartauto.v1, syntax proto3)
/* eslint-disable */

import type { GenFile, GenMessage, GenService } from "@bufbuild/protobuf/codegenv1";
import { fileDesc, messageDesc, serviceDesc } from "@bufbuild/protobuf/codegenv1";
import type { Message } from "@bufbuild/protobuf";

/**
 * Describes the file smartauto/v1/entity_sevice.proto.
 */
export const file_smartauto_v1_entity_sevice: GenFile = /*@__PURE__*/
  fileDesc("CiBzbWFydGF1dG8vdjEvZW50aXR5X3NldmljZS5wcm90bxIMc21hcnRhdXRvLnYxIhUKE0NyZWF0ZUVudGl0eVJlcXVlc3QiFgoUQ3JlYXRlRW50aXR5UmVzcG9uc2UyZgoNRW50aXR5U2VydmljZRJVCgxDcmVhdGVFbnRpdHkSIS5zbWFydGF1dG8udjEuQ3JlYXRlRW50aXR5UmVxdWVzdBoiLnNtYXJ0YXV0by52MS5DcmVhdGVFbnRpdHlSZXNwb25zZWIGcHJvdG8z");

/**
 * @generated from message smartauto.v1.CreateEntityRequest
 */
export type CreateEntityRequest = Message<"smartauto.v1.CreateEntityRequest"> & {
};

/**
 * Describes the message smartauto.v1.CreateEntityRequest.
 * Use `create(CreateEntityRequestSchema)` to create a new message.
 */
export const CreateEntityRequestSchema: GenMessage<CreateEntityRequest> = /*@__PURE__*/
  messageDesc(file_smartauto_v1_entity_sevice, 0);

/**
 * @generated from message smartauto.v1.CreateEntityResponse
 */
export type CreateEntityResponse = Message<"smartauto.v1.CreateEntityResponse"> & {
};

/**
 * Describes the message smartauto.v1.CreateEntityResponse.
 * Use `create(CreateEntityResponseSchema)` to create a new message.
 */
export const CreateEntityResponseSchema: GenMessage<CreateEntityResponse> = /*@__PURE__*/
  messageDesc(file_smartauto_v1_entity_sevice, 1);

/**
 * @generated from service smartauto.v1.EntityService
 */
export const EntityService: GenService<{
  /**
   * @generated from rpc smartauto.v1.EntityService.CreateEntity
   */
  createEntity: {
    methodKind: "unary";
    input: typeof CreateEntityRequestSchema;
    output: typeof CreateEntityResponseSchema;
  },
}> = /*@__PURE__*/
  serviceDesc(file_smartauto_v1_entity_sevice, 0);
