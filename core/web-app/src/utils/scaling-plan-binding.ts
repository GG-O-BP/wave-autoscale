import { ScalingPlanDefinition } from '@/types/bindings/scaling-plan-definition';
import JSYaml from 'js-yaml';

export function generateScalingPlanDefinition({
  kind,
  id,
  db_id,
  metadata,
  plans,
  enabled,
}: {
  kind: string;
  id?: string;
  db_id?: string;
  metadata?: any;
  plans?: any[];
  enabled?: boolean;
}) {
  return {
    kind: kind,
    id,
    db_id,
    metadata: metadata ?? {},
    plans: plans ?? [],
    enabled,
  } as ScalingPlanDefinition;
}

export function serializeScalingPlanDefinition(
  scalingPlanDefinition: ScalingPlanDefinition
) {
  const { kind, id, metadata, plans, enabled } = scalingPlanDefinition;
  const editedPlans = plans?.map((planItem) => {
    const { ui, ...rest } = planItem;
    return {
      ...rest,
    };
  });
  const serialized = JSYaml.dump({
    kind,
    id,
    metadata,
    plans: editedPlans,
    enabled,
  });
  return serialized;
}

export function serializeScalingPlanDefinitions(
  scalingPlanDefinitions: ScalingPlanDefinition[]
) {
  const serialized = scalingPlanDefinitions.map((scalingPlanDefinition) =>
    serializeScalingPlanDefinition(scalingPlanDefinition)
  );
  const result = serialized.join('\n---\n');
  return result;
}

export function deserializeScalingPlanDefinition(serialized: string) {
  let deserialized: ScalingPlanDefinition = JSYaml.load(
    serialized
  ) as ScalingPlanDefinition;
  return deserialized;
}
