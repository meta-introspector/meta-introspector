% bott8_optimal_layout.mzn
% Find optimal 8D layout for distributed intelligence network
% Bott[8] manifold embedding with real-world data sources

include "globals.mzn";

% --- Parameters ---

% Number of nodes in the network
int: num_nodes;

% Node types (each node has exactly one type)
int: NUM_TYPES = 8;
set of int: TYPES = 1..NUM_TYPES;
array[TYPES] of string: type_names = [
    "LLM",           % 1: AI language models
    "User",          % 2: Human users
    "DAO",           % 3: Decentralized autonomous orgs
    "Blockchain",    % 4: Distributed ledgers
    "WikiData",      % 5: Knowledge graphs
    "OSM",           % 6: OpenStreetMap spatial data
    "Twitter",       % 7: Social media streams
    "Mycelium"       % 8: Fungal network substrate
];

% 8D Bott manifold coordinates (one per dimension)
int: NUM_DIMS = 8;
set of int: DIMS = 1..NUM_DIMS;
array[DIMS] of string: dim_names = [
    "Real",          % 1: R (real line)
    "Complex",       % 2: C (complex plane)
    "Quaternion",    % 3: H (quaternions)
    "Octonion",      % 4: O (octonions)
    "Time",          % 5: T (temporal)
    "Information",   % 6: I (bits/entropy)
    "Social",        % 7: S (network effects)
    "Semantic"       % 8: M (meaning/memes)
];

% Decision variables: node positions in 8D space
array[1..num_nodes, DIMS] of var -100..100: position;

% Decision variables: node type assignment
array[1..num_nodes] of var TYPES: node_type;

% --- Input Data (from .dzn file) ---

% Node properties
array[1..num_nodes] of string: node_names;
array[1..num_nodes] of int: node_capacity;      % Processing capacity
array[1..num_nodes] of int: node_latency;       % Response time (ms)
array[1..num_nodes] of float: node_reliability; % Uptime 0.0-1.0

% Connection matrix (1 if nodes can connect, 0 otherwise)
array[1..num_nodes, 1..num_nodes] of 0..1: can_connect;

% Type affinity matrix (how well types work together)
array[TYPES, TYPES] of float: type_affinity;

% Dimension weights for each type (how important each dimension is)
array[TYPES, DIMS] of float: type_dim_weights;

% Target distribution (how many nodes of each type we want)
array[TYPES] of int: target_type_count;

% --- Constraints ---

% 1. Type distribution: match target counts
constraint forall(t in TYPES) (
    sum(n in 1..num_nodes)(bool2int(node_type[n] == t)) = target_type_count[t]
);

% 2. Spatial clustering: nodes of same type should be closer
% Distance metric: Manhattan distance in 8D space
array[1..num_nodes, 1..num_nodes] of var 0..1600: distance;
constraint forall(i, j in 1..num_nodes where i < j) (
    distance[i,j] = sum(d in DIMS)(abs(position[i,d] - position[j,d]))
);

% 3. Type affinity: high-affinity types should be closer
constraint forall(i, j in 1..num_nodes where i < j /\ can_connect[i,j] = 1) (
    distance[i,j] <= 100 + 50 * (1.0 - type_affinity[node_type[i], node_type[j]])
);

% 4. Dimension alignment: nodes should occupy dimensions matching their type
constraint forall(n in 1..num_nodes, d in DIMS) (
    type_dim_weights[node_type[n], d] > 0.5 -> abs(position[n,d]) >= 10
);

% 5. Mycelium substrate: at least one Mycelium node at origin
constraint exists(n in 1..num_nodes) (
    node_type[n] = 8 /\ forall(d in DIMS)(position[n,d] = 0)
);

% 6. Bott periodicity: enforce 8-fold symmetry
% Nodes should distribute evenly across octants
array[1..8] of var 0..num_nodes: octant_count;
constraint forall(oct in 1..8) (
    octant_count[oct] = sum(n in 1..num_nodes)(
        bool2int(
            (position[n,1] >= 0 /\ position[n,2] >= 0 /\ position[n,3] >= 0) = (oct mod 2 = 1) /\
            (position[n,4] >= 0 /\ position[n,5] >= 0) = ((oct div 2) mod 2 = 1) /\
            (position[n,6] >= 0 /\ position[n,7] >= 0) = ((oct div 4) mod 2 = 1)
        )
    )
);

% Octants should be balanced (within 20% of average)
constraint forall(oct in 1..8) (
    octant_count[oct] >= (num_nodes div 8) - (num_nodes div 40) /\
    octant_count[oct] <= (num_nodes div 8) + (num_nodes div 40)
);

% --- Objective Function ---

% Minimize total network cost (distance weighted by capacity and latency)
var float: total_distance_cost = sum(i, j in 1..num_nodes where i < j /\ can_connect[i,j] = 1) (
    distance[i,j] * (node_latency[i] + node_latency[j]) / (node_capacity[i] + node_capacity[j])
);

% Maximize type affinity (reward high-affinity connections)
var float: total_affinity = sum(i, j in 1..num_nodes where i < j /\ can_connect[i,j] = 1) (
    type_affinity[node_type[i], node_type[j]] * (200 - distance[i,j])
);

% Maximize reliability-weighted coverage
var float: total_reliability = sum(n in 1..num_nodes) (
    node_reliability[n] * node_capacity[n]
);

% Combined objective (weights tunable)
var float: objective = 
    total_affinity * 2.0 +           % Prioritize affinity
    total_reliability * 1.0 -        % Reward reliability
    total_distance_cost * 0.5;       % Penalize distance

solve maximize objective;

% --- Output ---

output [
    "% Optimal 8D Bott[8] Layout\n",
    "% Objective: ", show(objective), "\n",
    "% Total Affinity: ", show(total_affinity), "\n",
    "% Total Reliability: ", show(total_reliability), "\n",
    "% Total Distance Cost: ", show(total_distance_cost), "\n\n"
] ++ [
    "% Node ", show(n), ": ", node_names[n], " (", type_names[node_type[n]], ")\n",
    "%   Position: [",
    show(position[n,1]), ", ", show(position[n,2]), ", ", show(position[n,3]), ", ", 
    show(position[n,4]), ", ", show(position[n,5]), ", ", show(position[n,6]), ", ", 
    show(position[n,7]), ", ", show(position[n,8]), "]\n",
    "%   Capacity: ", show(node_capacity[n]), 
    ", Latency: ", show(node_latency[n]), 
    ", Reliability: ", show(node_reliability[n]), "\n\n"
    | n in 1..num_nodes
] ++ [
    "\n% Octant Distribution:\n"
] ++ [
    "% Octant ", show(oct), ": ", show(octant_count[oct]), " nodes\n"
    | oct in 1..8
];
