import { useCallback, useMemo } from "react";
import {
  useGetAllParticipants,
  useGetMyself,
} from "@/data/orval/participants/participants";
import type { ParticipantDto } from "@/data/orval/model";
import type { ParticipantResolverFn } from "@eunomia/ui";

export function useHeimdallDirectory() {
  const { data: myselfRes } = useGetMyself({
    query: { staleTime: 5 * 60_000, retry: false },
  });
  const { data: allRes } = useGetAllParticipants({
    query: { staleTime: 5 * 60_000 },
  });

  const self = myselfRes?.status === 200 ? myselfRes.data : undefined;

  const byId = useMemo(() => {
    const map = new Map<string, ParticipantDto>();
    const list = allRes?.status === 200 ? allRes.data : [];
    for (const p of list) {
      map.set(p.participant_id, p);
    }
    if (self) {
      map.set(self.participant_id, self);
    }
    return map;
  }, [allRes, self]);

  const isSelf = useCallback(
    (id?: string | null) => Boolean(id && self && id === self.participant_id),
    [self],
  );

  const resolveParticipant: ParticipantResolverFn = useCallback(
    (id: string) => {
      const found = byId.get(id);
      return {
        nick: found?.participant_nick,
        isSelf: isSelf(id),
      };
    },
    [byId, isSelf],
  );

  return { self, byId, isSelf, resolveParticipant };
}
