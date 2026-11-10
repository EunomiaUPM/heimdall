import { getApiGatewayBase } from "@eunomia/ui";
import type {
  EventEnvelope,
  EventLink,
  EventTopicCatalog,
  LiveInvalidations,
} from "@eunomia/ui/events";

/** Heimdall publishes `heimdall:<entity>:<action>`; the entity is the area shown. */
const ENTITIES: Record<string, string> = {
  grant: "Credential request",
  issuance: "Issuance",
  interaction: "Interaction",
  verification: "Verification",
  participant: "Participant",
  vc_status: "Issued credential",
};
const ACTIONS: Record<string, string> = {
  create: "created",
  edit: "updated",
  delete: "deleted",
  revoke: "revoked",
  suspend: "suspended",
  reinstate: "reinstated",
};

/** Readable labels and in-app links for heimdall's bus topics. */
export const EventTopics: EventTopicCatalog = {
  areas: Object.entries(ENTITIES).map(([entity, label]) => ({
    pattern: `heimdall:${entity}:*`,
    label,
  })),

  /** `heimdall:grant:create` → `Credential request created`. */
  describe(topic: string): string {
    const [, entity, action] = topic.split(":");
    const subject = ENTITIES[entity] ?? entity ?? topic;
    const verb = action ? (ACTIONS[action] ?? action) : "";
    return `${subject} ${verb}`.trim();
  },

  area(topic: string): string {
    const entity = topic.split(":")[1];
    return ENTITIES[entity] ?? entity ?? topic;
  },

  /** The request or participant page the event is about. */
  link(event: EventEnvelope): EventLink | undefined {
    const p = (event.payload ?? {}) as Record<string, unknown>;
    if (event.topic.startsWith("heimdall:participant:") && typeof p.participant_id === "string")
      return {
        to: "/participants/$participantId",
        params: { participantId: p.participant_id },
        label: p.participant_id,
      };
    if (event.topic.startsWith("heimdall:vc_status:") && typeof p.issuance_id === "string")
      return { to: "/issued/$issuedId", params: { issuedId: p.issuance_id }, label: p.issuance_id };
    if (typeof p.grant_id === "string")
      return { to: "/requests/$requestId", params: { requestId: p.grant_id }, label: p.grant_id };
    return undefined;
  },
};

/** Query key prefixes (orval keys are request paths) each heimdall topic refreshes. */
export const EVENT_INVALIDATIONS: LiveInvalidations = [
  ["heimdall:grant:", ["/approver"]],
  ["heimdall:issuance:", ["/approver", "/issued"]],
  ["heimdall:interaction:", ["/approver"]],
  ["heimdall:verification:", ["/approver"]],
  ["heimdall:participant:", ["/minions"]],
  ["heimdall:vc_status:", ["/issued"]],
];

/** The bus's SSE stream under the configured API base. */
export const liveEventsUrl = (): string => {
  const base = getApiGatewayBase() || import.meta.env.VITE_API_PATH || "/api/v1";
  return `${base.replace(/\/$/, "")}/events/stream?topic=${encodeURIComponent("heimdall:**")}`;
};
