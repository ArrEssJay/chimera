// TypeScript Contract Definitions for Chimera Node Graph System
// ⚠️ LOCKED CONTRACT - DO NOT MODIFY WITHOUT TEAM APPROVAL
// All agents must implement against these interfaces

/**
 * Core node definition that describes a DSP processing node
 */
export interface NodeDefinition {
  /** Unique identifier for this node type */
  id: string;
  
  /** Human-readable name */
  name: string;
  
  /** Category for organization in palette */
  category: 'source' | 'processing' | 'analysis' | 'sink';
  
  /** Description of what this node does */
  description: string;
  
  /** Input port definitions */
  inputs: PortDefinition[];
  
  /** Output port definitions */
  outputs: PortDefinition[];
  
  /** Configurable parameters */
  parameters: ParameterDefinition[];
  
  /** Optional icon identifier */
  icon?: string;
}

/**
 * Port definition for node inputs/outputs
 */
export interface PortDefinition {
  /** Unique ID within this node */
  id: string;
  
  /** Human-readable name */
  name: string;
  
  /** Data type this port accepts/produces */
  type: DataType;
  
  /** Optional description */
  description?: string;
}

/**
 * Supported data types for node graph connections
 */
export type DataType = 
  | 'BitStream'      // Vec<bool>
  | 'IQData'         // Vec<Complex<f32>>
  | 'AudioSamples'   // Vec<f32>
  | 'Metadata';      // HashMap<String, String>

/**
 * Parameter definition for node configuration
 */
export interface ParameterDefinition {
  /** Unique ID within this node */
  id: string;
  
  /** Human-readable name */
  name: string;
  
  /** Parameter type */
  type: 'number' | 'string' | 'boolean' | 'enum';
  
  /** Default value */
  defaultValue: number | string | boolean;
  
  /** For number type: min/max constraints */
  min?: number;
  max?: number;
  
  /** For enum type: allowed values */
  options?: string[];
  
  /** Optional description */
  description?: string;
}

/**
 * Graph API for node graph manipulation
 */
export interface GraphAPI {
  /**
   * Create a new node instance
   * @returns Node ID
   */
  createNode(type: string, position: Position): Promise<string>;
  
  /**
   * Delete a node
   */
  deleteNode(nodeId: string): Promise<void>;
  
  /**
   * Connect two nodes
   */
  connectNodes(
    fromId: string, 
    fromPort: number, 
    toId: string, 
    toPort: number
  ): Promise<void>;
  
  /**
   * Disconnect two nodes
   */
  disconnectNodes(connectionId: string): Promise<void>;
  
  /**
   * Update node parameters
   */
  updateNodeParams(nodeId: string, params: Record<string, unknown>): Promise<void>;
  
  /**
   * Validate the graph structure
   */
  validateGraph(): Promise<ValidationResult>;
  
  /**
   * Execute the graph
   */
  executeGraph(): Promise<GraphResult>;
  
  /**
   * Get all available node types
   */
  getAvailableNodes(): Promise<NodeDefinition[]>;
  
  /**
   * Serialize graph to JSON
   */
  serializeGraph(): Promise<GraphJSON>;
  
  /**
   * Load graph from JSON
   */
  loadGraph(json: GraphJSON): Promise<void>;
}

/**
 * Position in 2D space
 */
export interface Position {
  x: number;
  y: number;
}

/**
 * Validation result
 */
export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export interface ValidationError {
  type: 'cycle' | 'type_mismatch' | 'disconnected' | 'invalid_param';
  nodeId?: string;
  message: string;
}

/**
 * Graph execution result
 */
export interface GraphResult {
  success: boolean;
  outputs: OutputData[];
  error?: string;
  executionTimeMs: number;
}

export interface OutputData {
  nodeId: string;
  portId: string;
  type: DataType;
  data: unknown; // Type depends on DataType
}

/**
 * Serialized graph format
 */
export interface GraphJSON {
  version: string;
  nodes: NodeJSON[];
  connections: ConnectionJSON[];
}

export interface NodeJSON {
  id: string;
  type: string;
  position: Position;
  params: Record<string, unknown>;
}

export interface ConnectionJSON {
  id: string;
  from: { nodeId: string; portIndex: number };
  to: { nodeId: string; portIndex: number };
}

/**
 * React Flow node data (for UI layer)
 */
export interface ReactFlowNodeData {
  definition: NodeDefinition;
  params: Record<string, unknown>;
}

/**
 * React Flow connection validation
 */
export type ConnectionValidator = (
  sourceType: DataType,
  targetType: DataType
) => boolean;

/**
 * Node inspector props
 */
export interface NodeInspectorProps {
  nodeId: string | null;
  definition: NodeDefinition | null;
  currentParams: Record<string, unknown>;
  onParamsChange: (params: Record<string, unknown>) => void;
}
