// Sample DOT files for ndot-client
// These samples are copied from the ndot directory

export const samples = {
  digraph: `digraph graphname {
    a -> b -> c;
    b -> d;
}`,

  label: `digraph {
    a [label="Foo"];
    a -> b
}`,

  shape: `digraph {
    a [label="Node A"];
    b [shape=box, label="Node B"];
    c [shape=diamond, label="Node C"];
    a -> b;
    b -> c;
    c -> a;
}`,

  fullDigraph: `digraph {
    a -> b[label="0.2",weight="0.2"];
    a -> c[label="0.4",weight="0.4"];
    c -> b[label="0.6",weight="0.6"];
    c -> e[label="0.6",weight="0.6"];
    e -> e[label="0.1",weight="0.1"];
    e -> b[label="0.7",weight="0.7"];
}`,

  largeDiamond: `digraph large_diamond {
    // Basic diamond structure
    A -> B;
    A -> C;
    B -> D;
    C -> D;
    
    // Extended diamond structure with multiple levels
    D -> E;
    D -> F;
    E -> G;
    F -> G;
    
    // Add more nodes to make it "large"
    A -> H;
    H -> I;
    I -> J;
    J -> G;
    
    B -> K;
    K -> L;
    L -> G;
    
    C -> M;
    M -> N;
    N -> G;
}`,

  largeGraphs: `graph {
    rankdir=LR; // Left to Right, instead of Top to Bottom
    a -- { b c d };
    b -- { c e };
    c -- { e f };
    d -- { f g };
    e -- h;
    f -- { h i j g };
    g -- k;
    h -- { o l };
    i -- { l m j };
    j -- { m n k };
    k -- { n r };
    l -- { o m };
    m -- { o p n };
    n -- { q r };
    o -- { s p };
    p -- { s t q };
    q -- { t r };
    r -- t;
    s -- z;
    t -- z;
}`,

  port: `digraph {
  node1:port1 -> node2:port5:nw;
}`,

  showingAPath: `graph {
    a -- b[color=red,penwidth=3.0];
    b -- c;
    c -- d[color=red,penwidth=3.0];
    d -- e;
    e -- f;
    a -- d;
    b -- d[color=red,penwidth=3.0];
    c -- f[color=red,penwidth=3.0];
}`,

  subgraphs: `digraph {
    subgraph cluster_0 {
        label="Subgraph A";
        a -> b;
        b -> c;
        c -> d;
    }

    subgraph cluster_1 {
        label="Subgraph B";
        a -> f;
        f -> c;
    }
}`,
};

export type SampleKey = keyof typeof samples;

export const sampleNames: Record<SampleKey, string> = {
  digraph: 'Simple Digraph',
  label: 'Node with Label',
  shape: 'Different Node Shapes',
  fullDigraph: 'Weighted Digraph',
  largeDiamond: 'Large Diamond Structure',
  largeGraphs: 'Large Undirected Graph',
  port: 'Port Connections',
  showingAPath: 'Highlighted Path',
  subgraphs: 'Subgraphs Example',
};
