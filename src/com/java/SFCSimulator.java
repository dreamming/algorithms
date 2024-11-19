package com.java;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

class PhysicalNode {

    public static Random randvCPU = new Random();
    public static Random randDelay = new Random();
    private Integer nodeNum;
    private Integer vCPU;
    private Integer remainVCPU;
    public Integer toDelay;
    public PhysicalNode(Integer nodeNum){
        // rand 3 ~ 15 vCPU
        int vCPU = randvCPU.ints(3, 16).findAny().getAsInt();
        // rand 3 ~ 10ms delay
        int delay = randDelay.ints(3, 11).findAny().getAsInt();
        this.toDelay = delay;
        this.vCPU = vCPU;
        this.remainVCPU = vCPU;
        this.nodeNum = nodeNum;
    }

    public String physicalNodeResources(){
        return "Node"+nodeNum +" -> "+this.remainVCPU +" vCPUs";
    }

    public String realNodeNumInfo(){
        Integer realNodeNum = nodeNum + 1;
        return "Node"+realNodeNum;
    }

    public Integer getVCPU() {
        return this.vCPU;
    }

    public Integer getNodeNum() {
        return this.nodeNum;
    }

}

class Graph {
    private Integer vertex;
    private Integer edge;
    private List<PhysicalNode>[] adj;
    public Graph(Integer v) {
        this.vertex = v;
        this.edge = 0;
        adj = new ArrayList[v];
        for (int i = 0; i < v; i++) {
            adj[i] = new ArrayList<PhysicalNode>();
        }
    }
    public void addEdge(PhysicalNode source,PhysicalNode target) {
        Integer sv = source.getNodeNum();
        // Integer tv = target.getNodeNum();
        adj[sv].add(target);
        // adj[tv].add(source);
        edge++;
    }

    public List<PhysicalNode> adj(Integer v) {
        return adj[v];
    }


    // public void DFS() {
    //     boolean[] visited = new boolean[vertex];
    //     for (int i = 0; i < vertex; i++) {
    //         if (!visited[i]) {
    //             DFSUtil(i, visited);
    //         }
    //     }
    // }
    // private void DFSUtil(int vertex, boolean[] visited) {
    //     visited[vertex] = true;
    //     System.out.print(vertex + " ");
    //     for (PhysicalNode adj : adj[vertex]) {
    //         if (!visited[adj.getNodeNum()]) {
    //             DFSUtil(adj.getNodeNum(), visited);
    //         }
    //     }
    // }

    public void physicalNodeLinkDelay(){
        for (int i = 0; i < vertex; i++) {
            int vertexNodeNum = i +1;
            for (PhysicalNode physicalNode : adj[i]) {
                String linkDelay = "Node"+vertexNodeNum + " -> " + physicalNode.realNodeNumInfo() + ": "+physicalNode.toDelay+" ms";
                System.out.println(linkDelay);
            }
        }
    }

    public Integer V() {
        return vertex;
    }
    public Integer E() {
        return edge / 2;
    }

}

public class SFCSimulator {

    private PhysicalNode[] physicalNodes;
    private Graph infrastructure ;

    private PhysicalNode[] initPhysicalNode(){
        PhysicalNode[] nodes = new PhysicalNode[15];
        for (int i = 0; i < 15; i++) {
            PhysicalNode node = new PhysicalNode(i);
            nodes[i] = node;
        }
        return nodes;
    }
    public void initInfrastructure(){
        PhysicalNode[] nodes = initPhysicalNode();
        Graph graph = new Graph(15);
        // for vertex 1
        graph.addEdge(nodes[0], nodes[1]);
        graph.addEdge(nodes[0], nodes[5]);
        graph.addEdge(nodes[0], nodes[2]);
        graph.addEdge(nodes[0], nodes[7]);
        // for vertex 2
        graph.addEdge(nodes[1], nodes[0]);
        graph.addEdge(nodes[1], nodes[2]);
        graph.addEdge(nodes[1], nodes[3]);
        // for vertex 3
        graph.addEdge(nodes[2], nodes[0]);
        graph.addEdge(nodes[2], nodes[1]);
        graph.addEdge(nodes[2], nodes[6]);
        // for vertex 4
        graph.addEdge(nodes[3], nodes[1]);
        graph.addEdge(nodes[3], nodes[4]);
        graph.addEdge(nodes[3], nodes[10]);
        // for vertex 5
        graph.addEdge(nodes[4], nodes[3]);
        graph.addEdge(nodes[4], nodes[6]);
        // for vertex 6
        graph.addEdge(nodes[5], nodes[0]); 
        graph.addEdge(nodes[5], nodes[9]);
        // for vertex 7
        graph.addEdge(nodes[6], nodes[2]);
        graph.addEdge(nodes[6], nodes[4]);
        graph.addEdge(nodes[6], nodes[7]);
        graph.addEdge(nodes[6], nodes[8]);
        // for vertex 8
        graph.addEdge(nodes[7], nodes[0]);
        graph.addEdge(nodes[7], nodes[6]);
        graph.addEdge(nodes[7], nodes[11]);
        // for vertex 9
        graph.addEdge(nodes[8], nodes[9]);
        graph.addEdge(nodes[8], nodes[6]);
        graph.addEdge(nodes[8], nodes[12]);
        // for vertex 10
        graph.addEdge(nodes[9], nodes[5]);
        graph.addEdge(nodes[9], nodes[8]);
        graph.addEdge(nodes[9], nodes[13]);
        // for vertex 11
        graph.addEdge(nodes[10], nodes[3]);
        graph.addEdge(nodes[10], nodes[11]);
        graph.addEdge(nodes[10], nodes[14]);
        // for vertex 12
        graph.addEdge(nodes[11], nodes[7]);
        graph.addEdge(nodes[11], nodes[10]);
        graph.addEdge(nodes[11], nodes[12]);
        graph.addEdge(nodes[11], nodes[13]);
        // for vertext 13
        graph.addEdge(nodes[12], nodes[8]);
        graph.addEdge(nodes[12], nodes[11]);
        graph.addEdge(nodes[12], nodes[13]);
        // for vertex 14
        graph.addEdge(nodes[13], nodes[9]);
        graph.addEdge(nodes[13], nodes[12]);
        graph.addEdge(nodes[13], nodes[14]);
        // for vertex 15
        graph.addEdge(nodes[14], nodes[10]);
        graph.addEdge(nodes[14], nodes[11]);
        graph.addEdge(nodes[14], nodes[13]);

        this.infrastructure = graph;
        this.physicalNodes = nodes;
    }

    public Graph getInfrastructure(){
        return this.infrastructure;
    }

    public PhysicalNode[] getPhysicalNodes(){
        return this.physicalNodes;
    }
    

    public static void main(String[] args) {

        SFCSimulator simulator = new SFCSimulator();
        simulator.initInfrastructure();

        PhysicalNode[] physicalNodes = simulator.getPhysicalNodes();
        for (PhysicalNode physicalNode : physicalNodes) {
            System.out.println(physicalNode.physicalNodeResources());
        }
        Graph graph = simulator.getInfrastructure();
        System.out.println("Graph vertex: " + graph.V());
        System.out.println("Graph edge: " + graph.E());
        graph.physicalNodeLinkDelay();
    }

    
}