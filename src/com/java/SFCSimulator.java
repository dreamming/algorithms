package com.java;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Deque;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Comparator;
import java.util.Random;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

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

    public Integer remainVCPU() {
        return this.remainVCPU;
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
        // for vertex 13
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


    public void savePhysicalInfraBeforePlacement(){

    }

    public void savePhysicalInfraAfterPlacement(){

    }

    public void savePhysicalLinkDelay(){

    }


    public Map<Boolean,List<String>> simulate3SFC(){
        boolean[] result = new boolean[3];
        Random randVNFs = new Random();
        Random randDelay = new Random();
        Random randvCPUs = new Random();
        List<String> resultDesc = new ArrayList<>();
        for (int i = 1; i <= 3; i++) {
            int delay = randDelay.ints(10, 16).findAny().getAsInt();
            int vnfs = randVNFs.ints(3, 8).findAny().getAsInt();
            StringBuilder sBuilder = new StringBuilder();
            String sfc = "SFC"+i;
            sBuilder.append(sfc+":\n");
            String vnfDes = sfc +" have " + vnfs +" vNFs, ";
            sBuilder.append(vnfDes);
            Deque<Integer> dq = new ArrayDeque<>();
            dq.add(0);
            for (int j = 1; j <= vnfs; j++) {
                int vCPU = randvCPUs.ints(1, 4).findAny().getAsInt();
                dq.add(vCPU);
                String vCPUDesc = "vNF"+j+" requires "+vCPU+" vCPUs";
                sBuilder.append(vCPUDesc);
                if(j == vnfs){
                    sBuilder.append(". \n");
                } else {
                    sBuilder.append(", ");
                }
            }
            String depayThreshold = "Delay threshold for "+sfc+" = " + delay +" ms";
            sBuilder.append(depayThreshold);
            sBuilder.append("\n");

            Integer miniDelay = simulateSFC(vnfs,null,i-1);

            if (miniDelay <= delay && miniDelay >0 ) {
                result[i-1] = true;
            }

            String miniDelayDesc = "Delay on the shortest path = " + miniDelay +" ms";
            sBuilder.append(miniDelayDesc);
            sBuilder.append("\n");
            if (delay < miniDelay) {
                String invalidDesc = "there is no placement for "+sfc;
                sBuilder.append(invalidDesc);
            } else {
                // TODO graph top
                sBuilder.append("need show graph top");
            }

            sBuilder.append("\n\r");
            resultDesc.add(sBuilder.toString());
        }

        Map<Boolean,List<String>> resultMap = new HashMap<>();
        if (result[0] && result[1] && result[2]) {
            resultMap.put(true, resultDesc);
        } else {
            resultMap.put(false, resultDesc);
        }
        return resultMap;
    }


    public void simulateSFC(List<Integer> sourceVertexs){
        Random randVNFs = new Random();
        Random randDelay = new Random();
        Random randvCPUs = new Random();
        for (int i = 0; i < sourceVertexs.size(); i++) {
            Integer sourceVertex = sourceVertexs.get(i);
            int delay = randDelay.ints(10, 16).findAny().getAsInt();
            int vnfs = randVNFs.ints(3, 8).findAny().getAsInt();
            String result = simulateSpecificSFC(vnfs,delay,sourceVertex,i+1,randvCPUs);
            System.out.println(result);
        }
    }



    public String simulateSpecificSFC(Integer vnfs,Integer thresholdDelay,Integer initVertex,Integer sfcNo,Random randvCPUs){
        StringBuilder sBuilder = new StringBuilder();
        String sfc = "SFC"+sfcNo;
        sBuilder.append(sfc+":\n");
        String vnfDes = sfc +" have " + vnfs +" vNFs, ";
        sBuilder.append(vnfDes);
        for (int j = 1; j <= vnfs; j++) {
            int vCPU = randvCPUs.ints(1, 4).findAny().getAsInt();
            String vCPUDesc = "vNF"+j+" requires "+vCPU+" vCPUs";
            sBuilder.append(vCPUDesc);
            if(j == vnfs){
                sBuilder.append(". \n");
            } else {
                sBuilder.append(", ");
            }
        }
        String depayThreshold = "Delay threshold for "+sfc+" = " + thresholdDelay +" ms";
        sBuilder.append(depayThreshold);
        sBuilder.append("\n");

        List<PhysicalNode> preNodeList = new ArrayList<>();
        Integer miniDelay = shortestPathDelay(initVertex,vnfs,preNodeList);
        String miniDelayDesc = "Delay on the shortest path = " + miniDelay +" ms";
        sBuilder.append(miniDelayDesc);
        sBuilder.append("\n");
        if (thresholdDelay < miniDelay || miniDelay < 0) {
            String invalidDesc = "there is no placement for "+sfc;
            sBuilder.append(invalidDesc);
        } else {
            String showPhysicalTop = IntStream.range(0, preNodeList.size()).mapToObj(index -> {
                PhysicalNode node = preNodeList.get(index);
                Integer num = node.getNodeNum() + 1;
                return "Node"+num+ " {NF" + (index+1) + "("+node.remainVCPU()+" vCPUs)}";
            }).collect(Collectors.joining(" -> "));
            // Integer sourceNum = initVertex + 1;
            // sBuilder.append("Source {Node"+sourceNum+"} -> ").append(showPhysicalTop);
            sBuilder.append(showPhysicalTop);
        }
        sBuilder.append("\n\r");
        return sBuilder.toString();
    }

    // Dijkstra's algorithm
    private Integer shortestPathDelay(Integer initVertex,Integer vNFs,List<PhysicalNode> preNodeList){
        Graph graph = getInfrastructure();
        PhysicalNode[] physicalNodes = getPhysicalNodes();
        Integer size = graph.V();
        int INF = 0x3f3f3f3f;
        int[] distance = new int[size];
        Arrays.fill(distance, INF);
        // PhysicalNode v = nodes[vertex];
        distance[initVertex] = 0;
        PriorityQueue<int[]> pq = new PriorityQueue<int[]>(size, Comparator.comparingInt(o -> o[2]));
        pq.add(new int[] {initVertex,0,vNFs+1});
        while (!pq.isEmpty()) {
            int[] node = pq.poll();
            Integer nodeNum = node[0];
            Integer miniDelay = node[1];
            Integer vnfs = node[2];
            PhysicalNode prePhysicalNode = physicalNodes[nodeNum];
            preNodeList.add(prePhysicalNode);
            if (vnfs > 0) {
                for (PhysicalNode adjPhysicalNode : graph.adj(nodeNum)) {
                    Integer adjNodeNum = adjPhysicalNode.getNodeNum();
                    Integer adjMiniDelay = adjPhysicalNode.toDelay + miniDelay;
                    if (adjMiniDelay < distance[adjNodeNum]) {
                        distance[adjNodeNum] = adjMiniDelay;
                        pq.add(new int[]{adjNodeNum,adjMiniDelay,vnfs-1});
                    }
                }
            }
        }
        return distance[size-1] == INF ? -1:distance[size-1];
    }
    

    public Integer simulateSFC(Integer vNFs, Deque dq, Integer vertex){
        // PhysicalNode[] nodes = getPhysicalNodes();
        Graph graph = getInfrastructure();
        Integer size = graph.V();
        int INF = 0x3f3f3f3f;
        int[] distance = new int[size];
        Arrays.fill(distance, INF);
        // PhysicalNode v = nodes[vertex];
        distance[vertex] = 0;
        PriorityQueue<int[]> pq = new PriorityQueue<int[]>(size, Comparator.comparingInt(o -> o[2]));
        pq.add(new int[] {vertex,0,vNFs+1});
        while (!pq.isEmpty()) {
            int[] node = pq.poll();
            Integer nodeNum = node[0];
            Integer miniDelay = node[1];
            Integer vnfs = node[2];
            if (vnfs > 0) {
                for (PhysicalNode adjPhysicalNode : graph.adj(nodeNum)) {
                    Integer adjNodeNum = adjPhysicalNode.getNodeNum();
                    Integer adjMiniDelay = adjPhysicalNode.toDelay + miniDelay;
                    if (adjMiniDelay < distance[adjNodeNum]) {
                        distance[adjNodeNum] = adjMiniDelay;
                        pq.add(new int[]{adjNodeNum,adjMiniDelay,vnfs-1});
                    }
                }
            }
        }
        return distance[size-1] == INF ? -1:distance[size-1];
    }


    public static void forShowAllValidSFCOutput(){
        while (true) {
            SFCSimulator simulator = new SFCSimulator();
            simulator.initInfrastructure();
            Map<Boolean,List<String>> result = simulator.simulate3SFC();
            Boolean resultBoolean = result.keySet().stream().findFirst().get();
            if (resultBoolean) {
                List<String> resultDesc = result.values().stream().findFirst().get();
                for (String string : resultDesc) {
                    System.out.println(string);
                }
                return;
            } 
        }
    }
    
    

    public static void main(String[] args) {
        // while (true) {
            SFCSimulator simulator = new SFCSimulator();
            simulator.initInfrastructure();
            simulator.simulateSFC(Arrays.asList(0,1,2));
        //     Thread.sl
        // }


    //    String result =  simulator.simulateSFC(1,10,0,1,new Random());
    //    System.out.println(result);
        // Map<Boolean,List<String>> result = simulator.simulate3SFC();
        // List<String> resultDesc = result.values().stream().findFirst().get();
        // for (String desc : resultDesc) {
        //     System.out.println(desc);
        // }

        // forShowAllValidSFCOutput();        
        // SFC1(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay
        // SFC2(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay
        // SFC3(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay

        
    }

    
}