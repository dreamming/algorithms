package com.java;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.PriorityQueue;
import java.util.Comparator;
import java.util.Random;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class PhysicalNode {

    public static Random randvCPU = new Random();
    public static Random randDelay = new Random();
    private Integer nodeNum;
    private Integer vCPU;
    private Integer remainVCPU;

    public PhysicalNode(Integer nodeNum){
        // rand 3 ~ 15 vCPU
        int vCPU = randvCPU.ints(3, 16).findAny().getAsInt();
        this.vCPU = vCPU;
        this.remainVCPU = vCPU;
        this.nodeNum = nodeNum;
    }

    public String physicalNodeRemainVCPU(){
        return "Node"+ getNodeNum() +" -> "+this.remainVCPU +" vCPUs";
    }

    public String physicalNodeVCPU(){
        return "Node"+ getNodeNum() +" -> "+this.vCPU +" vCPUs";
    }

    public Integer getVCPU() {
        return this.vCPU;
    }

    public Integer remainVCPU() {
        return this.remainVCPU;
    }

    public void setRemainVCPU(Integer remainVCPU){
        this.remainVCPU = remainVCPU;
    }

    public Integer getInnerNodeNum() {
        return this.nodeNum;
    }

    public Integer getNodeNum(){
        return this.nodeNum + 1;
    }

}

class NodeEdge {

    private Integer toNodeNum;
    private Integer toDelay;

    public  NodeEdge(Integer to, Integer delay){
        this.toNodeNum = to;
        this.toDelay = delay;
    }

    public Integer getToNodeNum(){
        return this.toNodeNum;
    }

    public Integer getToDelay(){
        return this.toDelay;
    }
    
}

class SfcInfra {

    private static Random randDelay = new Random();
    private Integer vertex;
    private Integer edge;
    private List<NodeEdge>[] adj;
    private PhysicalNode[] physicalNodes;

    public SfcInfra(Integer v) {
        this.vertex = v;
        this.edge = 0;
        adj = new ArrayList[v];
        for (int i = 0; i < v; i++) {
            adj[i] = new ArrayList<NodeEdge>();
        }
        this.physicalNodes = initPhysicalNode(v);
        buildSfcInfra();
    }

    private void buildSfcInfra(){
        // for vertex 1
        this.addEdge(0, 1);
        this.addEdge(0, 2);
        this.addEdge(0, 5);
        this.addEdge(0, 7);
        // for vertex 2
        this.addEdge(1, 2);
        this.addEdge(1, 3);
        // for vertex 3
        this.addEdge(2, 6);
        // for vertex 4
        this.addEdge(3, 4);
        this.addEdge(3, 10);
        // for vertex 5
        this.addEdge(4, 6);
        // for vertex 6
        this.addEdge(5, 9);
        // for vertex 7
        this.addEdge(6, 7);
        this.addEdge(6, 8);
        // for vertex 8
        this.addEdge(7, 11);
        // for vertex 9
        this.addEdge(8, 9);
        this.addEdge(8, 12);
        // for vertex 10
        this.addEdge(9, 13);
        // for vertex 11
        this.addEdge(10, 12);
        this.addEdge(10, 14);
        // for vertex 12
        this.addEdge(11, 12);
        this.addEdge(11, 14);
        // for vertex 13
        this.addEdge(12, 13);
        // for vertex 14
        this.addEdge(13, 14);
        // for vertex 15
    }

    public PhysicalNode[] initPhysicalNode(Integer size){
        PhysicalNode[] nodes = new PhysicalNode[size];
        for (int i = 0; i < size; i++) {
            PhysicalNode node = new PhysicalNode(i);
            nodes[i] = node;
        }
        return nodes;
    }

    public void addEdge(Integer sourceNodeNum, Integer targetNodeNum) {
        // rand 3 ~ 10ms delay
        Integer delay = randDelay.ints(3, 11).findAny().getAsInt();
        adj[sourceNodeNum].add(new NodeEdge(targetNodeNum, delay));
        adj[targetNodeNum].add(new NodeEdge(sourceNodeNum, delay));
        edge++;
    }

    public List<NodeEdge> neighborNodeEdge(Integer v) {
        return adj[v];
    }

    public List<String> oneWayNodeLinkDelay(){
        List<String> oneWayNodeLinkDelayDesc = new ArrayList<>();
        Set<String> printedEdges = new HashSet<>();
        for (int i = 0; i < vertex; i++) {
            for (NodeEdge nodeEdge : adj[i]) {
                Integer to = nodeEdge.getToNodeNum();
                String edgeKey = i < to ? i + "-" + to : to + "-" + i;
                if (!printedEdges.contains(edgeKey)) {
                String linkDelay = "Node"+(i+1) + " -> Node" + (nodeEdge.getToNodeNum()+1) + ": "+nodeEdge.getToDelay()+" ms";
                oneWayNodeLinkDelayDesc.add(linkDelay);
                printedEdges.add(edgeKey);
                }
            }
        }
        return oneWayNodeLinkDelayDesc;
    }

    public List<String> biWayNodeLinkDelay(){
        List<String> biWayNodeLinkDelayDesc = new ArrayList<>();
        for (int i = 0; i < vertex; i++) {
            for (NodeEdge nodeEdge : adj[i]) {
                String linkDelay = "Node"+(i+1) + " -> Node" + (nodeEdge.getToNodeNum()+1) + ": "+nodeEdge.getToDelay()+" ms";
                biWayNodeLinkDelayDesc.add(linkDelay);
            }
        }
        return biWayNodeLinkDelayDesc;
    }

    public List<String> physicalNodeCapacityCPU(){
        List<String> capacityCPUdesc = new ArrayList<>();
        for (PhysicalNode physicalNode : physicalNodes) {
            capacityCPUdesc.add(physicalNode.physicalNodeVCPU());
        }
        return capacityCPUdesc;
    }

    public List<String> physicalNodeRemainCPU(){
        List<String> remainCPUdesc = new ArrayList<>();
        for (PhysicalNode physicalNode : physicalNodes) {
            remainCPUdesc.add(physicalNode.physicalNodeRemainVCPU());
        }
        return remainCPUdesc;
    }

    public Integer V() {
        return vertex;
    }

    public Integer E() {
        return edge;
    }

    public PhysicalNode[] getPhysicalNodes(){
        return physicalNodes;
    }
}

public class SFCSimulator {

    private SfcInfra infrastructure ;

    public SFCSimulator(){
       this.infrastructure = new SfcInfra(15);
    }

    public SfcInfra getInfrastructure(){
        return this.infrastructure;
    }

    public PhysicalNode[] getPhysicalNodes(){
        return infrastructure.getPhysicalNodes();
    }

    public void showPhysicalNodeCapacityCPU(){
        for (String desc : infrastructure.physicalNodeCapacityCPU()) {
            System.out.println(desc);
        }
    }

    public void showPhysicalNodeRemainCPU(){
        for (String desc : infrastructure.physicalNodeRemainCPU()) {
            System.out.println(desc);
        }
    }

    public void showOneWayPhysicalNodeLinkDelay(){
        for (String desc : infrastructure.oneWayNodeLinkDelay()) {
            System.out.println(desc);
        }
    }

    public void showBiWayPhysicalNodeLinkDelay(){
        for (String desc : infrastructure.biWayNodeLinkDelay()) {
            System.out.println(desc);
        }
    }

    public void savePhysicalInfraBeforePlacement(){

    }

    public void savePhysicalInfraAfterPlacement(){

    }

    public void savePhysicalLinkDelay(){

    }

    public List<Map<Boolean,String>> simulateSFC(List<Integer> sourceVertexs){
        List<Map<Boolean,String>> allSFCResult = new ArrayList<>();
        Random randVNFs = new Random();
        Random randDelay = new Random();
        Random randvCPUs = new Random();
        for (int i = 0; i < sourceVertexs.size(); i++) {
            Integer sourceVertex = sourceVertexs.get(i);
            int delay = randDelay.ints(10, 16).findAny().getAsInt();
            int vnfs = randVNFs.ints(3, 8).findAny().getAsInt();
            Map<Boolean,String> result = simulateSpecificSFC(vnfs,delay,sourceVertex,i+1,randvCPUs);
            allSFCResult.add(result);
        }
        return allSFCResult;
    }


    public static SFCSimulator showValidSFC(List<Integer> sourceVertexs, Integer validSFCs){
        while (true) {
            SFCSimulator simulator = new SFCSimulator();
            List<Map<Boolean,String>> allSFCResult = simulator.simulateSFC(sourceVertexs);
            long trueKeyCount = allSFCResult.stream()
            .flatMap(map -> map.keySet().stream())
            .filter(key -> key)
            .count();
            // boolean allTrue = allSFCResult.stream().allMatch(map -> map.keySet().stream().allMatch(key -> key));
            if (trueKeyCount == validSFCs) {
                List<String> allStrings = allSFCResult.stream()
                .flatMap(map -> map.values().stream())
                .collect(Collectors.toList());
                for (String string : allStrings) {
                    System.out.println(string);
                }
                return simulator;
            }
        }
    }

    public Map<Boolean,String> simulateSpecificSFC(Integer vnfs,Integer thresholdDelay,Integer initVertex,Integer sfcNo, Random randvCPUs){
        Map<Boolean,String> result = new HashMap<>();
        StringBuilder sBuilder = new StringBuilder();
        String sfc = "SFC"+sfcNo;
        sBuilder.append(sfc+":\n");
        String vnfDes = sfc +" have " + vnfs +" vNFs, ";
        sBuilder.append(vnfDes);
        PriorityQueue<Integer> dq = new PriorityQueue<>(Comparator.reverseOrder());
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
        String depayThreshold = "Delay threshold for "+sfc+" = " + thresholdDelay +" ms";
        sBuilder.append(depayThreshold);
        sBuilder.append("\n");
        Integer size = getInfrastructure().V();
        PhysicalNode[] physicalNodes = getPhysicalNodes();
        List<PhysicalNode> preNodeList = new ArrayList<>();
        Integer[] predecessor = new Integer[size];
        Arrays.fill(predecessor, -1);
        Integer miniDelay = shortestPathDelay(initVertex,predecessor);
        buildPrePath(predecessor, size-1, preNodeList, physicalNodes);
        String miniDelayDesc = "Delay on the shortest path = " + miniDelay +" ms";
        sBuilder.append(miniDelayDesc);
        sBuilder.append("\n");
        if (thresholdDelay < miniDelay || miniDelay < 0) {
            String invalidDesc = "there is no placement for "+sfc;
            sBuilder.append(invalidDesc);
            sBuilder.append("\n\r");
            result.put(Boolean.FALSE,sBuilder.toString());
        } else {
            String showPhysicalTop = IntStream.range(0, preNodeList.size()).mapToObj(index -> {
                PhysicalNode node = preNodeList.get(index);
                Integer deductResource = Optional.ofNullable(dq.poll()).orElse(0);
                Integer num = node.getNodeNum();
                // Integer remainVCPU = node.remainVCPU() - deductResource;
                // node.setRemainVCPU(remainVCPU);
                // return "{Node "+num+ "} {NF" + (index+1) + "("+deductResource+" vCPUs)}";
                return "{Node "+num+ "}";
            }).collect(Collectors.joining(" -> "));
            sBuilder.append(showPhysicalTop);
            sBuilder.append("\n\r");
            result.put(Boolean.TRUE,sBuilder.toString());
        }
        return result;
    }

    private void buildPrePath(Integer[] predecessor, int current, List<PhysicalNode> prePath,PhysicalNode[] physicalNodes) {
        if (current == -1) {
            return;
        }
        buildPrePath(predecessor, predecessor[current],prePath,physicalNodes);
        prePath.add(physicalNodes[current]);
    }

    // Dijkstra's algorithm
    private Integer shortestPathDelay(Integer initVertex, Integer[] predecessor){
        SfcInfra sfcInfra = getInfrastructure();
        Integer size = sfcInfra.V();
        int INF = 0x3f3f3f3f;
        int[] distance = new int[size];
        Arrays.fill(distance, INF);
        distance[initVertex] = 0;
        PriorityQueue<int[]> pq = new PriorityQueue<int[]>(size, Comparator.comparingInt(o -> o[1]));
        pq.add(new int[] {initVertex,0});
        while (!pq.isEmpty()) {
            int[] node = pq.poll();
            Integer nodeNum = node[0];
            Integer miniDelay = node[1];
                for (NodeEdge adjNodeEdge : sfcInfra.neighborNodeEdge(nodeNum)) {
                    Integer adjNodeNum = adjNodeEdge.getToNodeNum();
                    Integer adjMiniDelay = adjNodeEdge.getToDelay() + miniDelay;
                    if (adjMiniDelay < distance[adjNodeNum]) {
                        distance[adjNodeNum] = adjMiniDelay;
                        predecessor[adjNodeNum] = nodeNum;
                        pq.add(new int[]{adjNodeNum,adjMiniDelay});
                    }
                }
        }
        return distance[size-1] == INF ? -1:distance[size-1];
    }

    public static void main(String[] args) {
            // SFCSimulator simulator = SFCSimulator.showValidSFC(Arrays.asList(0,1,2),3);
            // simulator.showBiWayPhysicalNodeLinkDelay();
            // System.out.println();
            // simulator.showPhysicalNodeCapacityCPU();
            // System.out.println();
            // simulator.showPhysicalNodeRemainCPU();
            // System.out.println();
            // real world
            SFCSimulator simulator = new SFCSimulator();
            List<Map<Boolean,String>> allSFCResult = simulator.simulateSFC(Arrays.asList(0,1,2));
            List<String> allStrings = allSFCResult.stream()
            .flatMap(map -> map.values().stream())
            .collect(Collectors.toList());
            for (String string : allStrings) {
                System.out.println(string);
            }

        // forShowAllValidSFCOutput();        
        // SFC1(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay
        // SFC2(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay
        // SFC3(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay

        
    }

    
}