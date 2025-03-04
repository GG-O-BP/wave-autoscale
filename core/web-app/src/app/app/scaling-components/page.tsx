'use client';

import dynamic from 'next/dynamic';
import { useEffect, useState } from 'react';
import { debounce } from 'lodash';
import { createColumnHelper } from '@tanstack/react-table';
import { renderKeyValuePairsWithJson } from '../common/keyvalue-renderer';
import EnabledBadge from '../common/enabled-badge';
import WASimpleTable from '../common/wa-simple-table';
import { ScalingComponentDefinition } from '@/types/bindings/scaling-component-definition';
import ScalingComponentService from '@/services/scaling-component';
import {
  deserializeScalingComponentDefinitions,
  serializeScalingComponentDefinitions,
} from '@/utils/scaling-component-binding';
import { PageSectionTitle } from '../common/page-section-title';
import PageHeader from '../common/page-header';

// Dynamic imports (because of 'window' object)
const YAMLEditor = dynamic(() => import('../common/yaml-editor'), {
  ssr: false,
});

// Table columns
const columnHelper = createColumnHelper<ScalingComponentDefinition>();
const columns = [
  columnHelper.accessor('id', {
    header: () => 'ID',
    cell: (cell) => cell?.getValue(),
  }),
  columnHelper.accessor('component_kind', {
    header: () => 'Component Kind',
    cell: (cell) => cell?.getValue(),
  }),
  columnHelper.accessor('metadata', {
    header: () => 'Metadata',
    cell: (cell) => (
      <span className="flex flex-col items-center break-all text-start">
        {renderKeyValuePairsWithJson(cell?.getValue(), true)}
      </span>
    ),
  }),
  columnHelper.accessor('enabled', {
    header: () => 'Enabled',
    cell: (cell) => <EnabledBadge enabled={cell?.getValue()} />,
  }),
];

// Service
async function getScalingComponents(): Promise<ScalingComponentDefinition[]> {
  const scalingComponents =
    await ScalingComponentService.getScalingComponents();
  return scalingComponents;
}

// Page
export default function ScalingComponentsPage() {
  const [yaml, setYaml] = useState<string>('');
  const [previewData, setPreviewData] = useState<ScalingComponentDefinition[]>(
    []
  );

  // Effects
  useEffect(() => {
    loadFromService();
  }, []);

  // Handlers
  const loadFromService = async () => {
    const scalingComponents = await getScalingComponents();
    // YAML
    try {
      const newYaml = serializeScalingComponentDefinitions(scalingComponents);
      setYaml(newYaml);
    } catch (error: any) {
      console.error(error);
      alert(error.message);
      return;
    }

    // Preview
    setPreviewData(scalingComponents);
  };

  const handleYamlChange = debounce((value: string) => {
    setYaml(value);

    let newPreviewData;
    try {
      newPreviewData = deserializeScalingComponentDefinitions(value);
    } catch (error: any) {
      console.error(error);
      alert(error.message);
      // TODO: Set annotations
      return;
    }

    try {
      setPreviewData(newPreviewData);
    } catch (error: any) {
      console.error(error);
      alert(error.message);
      return;
    }
  }, 500);

  const handleReset = () => {
    handleYamlChange.cancel();
    loadFromService();
  };

  const handleSave = async () => {
    try {
      const scalingComponents = deserializeScalingComponentDefinitions(yaml);
      const originalScalingComponents = await getScalingComponents();
      // Upsert
      const promises = scalingComponents.map((scalingComponentDefinition) => {
        return ScalingComponentService.createScalingComponent(
          scalingComponentDefinition
        );
      });
      // Delete
      const deletedScalingComponents = originalScalingComponents.filter(
        (originalScalingComponent) => {
          return !scalingComponents.some((scalingComponent) => {
            return scalingComponent.id === originalScalingComponent.id;
          });
        }
      );
      const deletePromises = deletedScalingComponents.map(
        (deletedScalingComponent) => {
          return ScalingComponentService.deleteScalingComponent(
            deletedScalingComponent.db_id
          );
        }
      );
      promises.push(...deletePromises);
      await Promise.all(promises);
    } catch (error: any) {
      console.error(error);
      alert(error.message);
      return;
    }
    loadFromService();
    alert('Saved');
  };

  return (
    <main className="flex h-full w-full flex-col">
      {/* Header */}
      <PageHeader title="Scaling Component Definitions" />

      {/* Sections */}
      <div className="min-height-0 flex w-full flex-1 space-x-8">
        {/* Preview */}
        <div className="flex h-full flex-1 flex-col overflow-y-auto">
          {/* Preview Title */}
          <div className="flex h-14 items-center px-6">
            <PageSectionTitle title="Preview" />
          </div>
          {/* Table */}
          <div className="px-6 pb-6">
            <WASimpleTable<ScalingComponentDefinition>
              tableOptions={{
                data: previewData,
                columns,
              }}
            />
          </div>
        </div>
        {/* Code */}
        <div className="flex h-full flex-1 flex-col bg-wa-gray-50 shadow-[-4px_0px_8px_rgba(23,25,28,0.08)]">
          {/* Code Title */}
          <div className="border-wa-gray-700 flex h-14 items-center border-b px-6">
            <div className="flex-1">
              <PageSectionTitle title="Code" />
            </div>
            <div className="flex items-center space-x-4">
              <button
                className="btn-ghost btn-sm btn flex h-8 items-center justify-center rounded-md text-sm"
                onClick={handleReset}
              >
                Reset Code
              </button>
              <button
                className="btn-gray btn-sm btn flex h-8 items-center justify-center rounded-md text-sm"
                onClick={handleSave}
              >
                Save
              </button>
            </div>
          </div>
          <YAMLEditor value={yaml} onChange={handleYamlChange} />
        </div>
      </div>
    </main>
  );
}
