// package com.java;

// import java.util.ArrayDeque;
// import java.util.ArrayList;
// import java.util.Arrays;
// import java.util.Collections;
// import java.util.Deque;
// import java.util.HashMap;
// import java.util.HashSet;
// import java.util.List;
// import java.util.Map;
// import java.util.Optional;
// import java.util.PriorityQueue;
// import java.util.Comparator;
// import java.util.Random;
// import java.util.Set;
// import java.util.stream.Collectors;
// import java.util.stream.IntStream;

// class PhysicalNode {

//     public static Random randvCPU = new Random();
//     public static Random randDelay = new Random();
//     private Integer nodeNum;
//     private Integer vCPU;
//     private Integer remainVCPU;
//     public Integer toDelay;
//     public PhysicalNode(Integer nodeNum){
//         // rand 3 ~ 15 vCPU
//         int vCPU = randvCPU.ints(3, 16).findAny().getAsInt();
//         // rand 3 ~ 10ms delay
//         int delay = randDelay.ints(3, 11).findAny().getAsInt();
//         this.toDelay = delay;
//         this.vCPU = vCPU;
//         this.remainVCPU = vCPU;
//         this.nodeNum = nodeNum;
//     }

//     public String physicalNodeRemainVCPU(){
//         return "Node"+nodeNum +" -> "+this.remainVCPU +" vCPUs";
//     }

//     public String physicalNodeVCPU(){
//         return "Node"+nodeNum +" -> "+this.vCPU +" vCPUs";
//     }

//     public String realNodeNumInfo(){
//         Integer realNodeNum = nodeNum;
//         return "Node"+realNodeNum;
//     }

//     public Integer getVCPU() {
//         return this.vCPU;
//     }

//     public Integer remainVCPU() {
//         return this.remainVCPU;
//     }

//     public void setRemainVCPU(Integer remainVCPU){
//         this.remainVCPU = remainVCPU;
//     }

//     public Integer getNodeNum() {
//         return this.nodeNum;
//     }

// }

// class NodeEdge{
//     private Integer toNodeNum;
//     private Integer toDelay;

//     public void NodeEdge(Integer to, Integer delay){
//         this.toNodeNum = to;
//         this.toDelay = delay;
//     } 
// }
 

// class Graph {
//     public static Random randDelay = new Random();
//     private Integer vertex;
//     private Integer edge;
//     private List<PhysicalNode>[] adj;
//     public Graph(Integer v) {
//         this.vertex = v;
//         this.edge = 0;
//         adj = new ArrayList[v];
//         for (int i = 0; i < v; i++) {
//             adj[i] = new ArrayList<PhysicalNode>();
//         }
//     }
//     public void addEdge(PhysicalNode source,PhysicalNode target) {
//         int delay = randDelay.ints(3, 11).findAny().getAsInt();
//         Integer from = source.getNodeNum();
//         Integer to = source.getNodeNum();
//         adj[from].add(new PhysicalNode(to, delay)); // 添加 from -> to
//         adj[to].add(new PhysicalNode(from, delay)); // 添加 to -> from
//         edge++;
//     }

//     public List<PhysicalNode> adj(Integer v) {
//         return adj[v];
//     }

//     public void physicalNodeLinkDelay(){
//         for (int i = 0; i < vertex; i++) {
//             for (PhysicalNode physicalNode : adj[i]) {
//                 String linkDelay = "Node"+i + " -> " + physicalNode.realNodeNumInfo() + ": "+physicalNode.toDelay+" ms";
//                 System.out.println(linkDelay);
//             }
//         }
//     }

//     public Integer V() {
//         return vertex;
//     }
//     public Integer E() {
//         return edge / 2;
//     }

// }

// public class SFCSimulator {

//     private PhysicalNode[] physicalNodes;
//     private Graph infrastructure ;

//     private PhysicalNode[] initPhysicalNode(){
//         PhysicalNode[] nodes = new PhysicalNode[15];
//         for (int i = 0; i < 15; i++) {
//             PhysicalNode node = new PhysicalNode(i);
//             nodes[i] = node;
//         }
//         return nodes;
//     }
//     public void initInfrastructure(){
//         PhysicalNode[] nodes = initPhysicalNode();
//         Graph graph = new Graph(15);
//         // for vertex 1
//         graph.addEdge(nodes[0], nodes[1]);
//         graph.addEdge(nodes[0], nodes[5]);
//         graph.addEdge(nodes[0], nodes[2]);
//         graph.addEdge(nodes[0], nodes[7]);
//         // for vertex 2
//         graph.addEdge(nodes[1], nodes[0]);
//         graph.addEdge(nodes[1], nodes[2]);
//         graph.addEdge(nodes[1], nodes[3]);
//         // for vertex 3
//         graph.addEdge(nodes[2], nodes[0]);
//         graph.addEdge(nodes[2], nodes[1]);
//         graph.addEdge(nodes[2], nodes[6]);
//         // for vertex 4
//         graph.addEdge(nodes[3], nodes[1]);
//         graph.addEdge(nodes[3], nodes[4]);
//         graph.addEdge(nodes[3], nodes[10]);
//         // for vertex 5
//         graph.addEdge(nodes[4], nodes[3]);
//         graph.addEdge(nodes[4], nodes[6]);
//         // for vertex 6
//         graph.addEdge(nodes[5], nodes[0]); 
//         graph.addEdge(nodes[5], nodes[9]);
//         // for vertex 7
//         graph.addEdge(nodes[6], nodes[2]);
//         graph.addEdge(nodes[6], nodes[4]);
//         graph.addEdge(nodes[6], nodes[7]);
//         graph.addEdge(nodes[6], nodes[8]);
//         // for vertex 8
//         graph.addEdge(nodes[7], nodes[0]);
//         graph.addEdge(nodes[7], nodes[6]);
//         graph.addEdge(nodes[7], nodes[11]);
//         // for vertex 9
//         graph.addEdge(nodes[8], nodes[9]);
//         graph.addEdge(nodes[8], nodes[6]);
//         graph.addEdge(nodes[8], nodes[12]);
//         // for vertex 10
//         graph.addEdge(nodes[9], nodes[5]);
//         graph.addEdge(nodes[9], nodes[8]);
//         graph.addEdge(nodes[9], nodes[13]);
//         // for vertex 11
//         graph.addEdge(nodes[10], nodes[3]);
//         graph.addEdge(nodes[10], nodes[11]);
//         graph.addEdge(nodes[10], nodes[14]);
//         // for vertex 12
//         graph.addEdge(nodes[11], nodes[7]);
//         graph.addEdge(nodes[11], nodes[10]);
//         graph.addEdge(nodes[11], nodes[12]);
//         graph.addEdge(nodes[11], nodes[13]);
//         // for vertex 13
//         graph.addEdge(nodes[12], nodes[8]);
//         graph.addEdge(nodes[12], nodes[11]);
//         graph.addEdge(nodes[12], nodes[13]);
//         // for vertex 14
//         graph.addEdge(nodes[13], nodes[9]);
//         graph.addEdge(nodes[13], nodes[12]);
//         graph.addEdge(nodes[13], nodes[14]);
//         // for vertex 15
//         graph.addEdge(nodes[14], nodes[10]);
//         graph.addEdge(nodes[14], nodes[11]);
//         graph.addEdge(nodes[14], nodes[13]);

//         this.infrastructure = graph;
//         this.physicalNodes = nodes;
//     }

//     public Graph getInfrastructure(){
//         return this.infrastructure;
//     }

//     public PhysicalNode[] getPhysicalNodes(){
//         return this.physicalNodes;
//     }

//     public void showPhysicalNodeCapacityCPU(){
//         for (PhysicalNode physicalNode : physicalNodes) {
//             System.out.println(physicalNode.physicalNodeVCPU());
//         }
//     }


//     public void showPhysicalNodeRemainCPU(){
//         for (PhysicalNode physicalNode : physicalNodes) {
//             System.out.println(physicalNode.physicalNodeRemainVCPU());
//         }
//     }

//     public void showPhysicalNodeLinkDelay(){
//         getInfrastructure().physicalNodeLinkDelay();
//     }


//     public void savePhysicalInfraBeforePlacement(){

//     }

//     public void savePhysicalInfraAfterPlacement(){

//     }

//     public void savePhysicalLinkDelay(){

//     }

//     public List<Map<Boolean,String>> simulateSFC(List<Integer> sourceVertexs){
//         List<Map<Boolean,String>> allSFCResult = new ArrayList<>();
//         Random randVNFs = new Random();
//         Random randDelay = new Random();
//         Random randvCPUs = new Random();
//         for (int i = 0; i < sourceVertexs.size(); i++) {
//             Integer sourceVertex = sourceVertexs.get(i);
//             int delay = randDelay.ints(10, 16).findAny().getAsInt();
//             int vnfs = randVNFs.ints(3, 8).findAny().getAsInt();
//             Map<Boolean,String> result = simulateSpecificSFC(vnfs,delay,sourceVertex,i+1,randvCPUs);
//             allSFCResult.add(result);
//         }
//         return allSFCResult;
//     }


//     public void showValidSFC(List<Integer> sourceVertexs, Integer validSFCs){
//         while (true) {
//             SFCSimulator simulator = new SFCSimulator();
//             simulator.initInfrastructure();
//             List<Map<Boolean,String>> allSFCResult = simulator.simulateSFC(sourceVertexs);
//             long trueKeyCount = allSFCResult.stream()
//             .flatMap(map -> map.keySet().stream())
//             .filter(key -> key)
//             .count();
//             // boolean allTrue = allSFCResult.stream().allMatch(map -> map.keySet().stream().allMatch(key -> key));
//             if (trueKeyCount == validSFCs) {
//                 List<String> allStrings = allSFCResult.stream()
//                 .flatMap(map -> map.values().stream())
//                 .collect(Collectors.toList());
//                 for (String string : allStrings) {
//                     System.out.println(string);
//                 }
//                 return;
//             }
//         }
//     }

//     public Map<Boolean,String> simulateSpecificSFC(Integer vnfs,Integer thresholdDelay,Integer initVertex,Integer sfcNo, Random randvCPUs){
//         Map<Boolean,String> result = new HashMap<>();
//         StringBuilder sBuilder = new StringBuilder();
//         String sfc = "SFC"+sfcNo;
//         sBuilder.append(sfc+":\n");
//         String vnfDes = sfc +" have " + vnfs +" vNFs, ";
//         sBuilder.append(vnfDes);
//         PriorityQueue<Integer> dq = new PriorityQueue<>(Comparator.reverseOrder());
//         for (int j = 1; j <= vnfs; j++) {
//             int vCPU = randvCPUs.ints(1, 4).findAny().getAsInt();
//             dq.add(vCPU);
//             String vCPUDesc = "vNF"+j+" requires "+vCPU+" vCPUs";
//             sBuilder.append(vCPUDesc);
//             if(j == vnfs){
//                 sBuilder.append(". \n");
//             } else {
//                 sBuilder.append(", ");
//             }
//         }
//         String depayThreshold = "Delay threshold for "+sfc+" = " + thresholdDelay +" ms";
//         sBuilder.append(depayThreshold);
//         sBuilder.append("\n");
//         Integer size = getInfrastructure().V();
//         PhysicalNode[] physicalNodes = getPhysicalNodes();
//         List<PhysicalNode> preNodeList = new ArrayList<>();
//         Integer[] predecessor = new Integer[size];
//         Arrays.fill(predecessor, -1);
//         Integer miniDelay = shortestPathDelay(initVertex,vnfs,dq,predecessor);
//         buildPrePath(predecessor, size-1, preNodeList, physicalNodes);
//         String miniDelayDesc = "Delay on the shortest path = " + miniDelay +" ms";
//         sBuilder.append(miniDelayDesc);
//         sBuilder.append("\n");
//         if (thresholdDelay < miniDelay || miniDelay < 0) {
//             String invalidDesc = "there is no placement for "+sfc;
//             sBuilder.append(invalidDesc);
//             sBuilder.append("\n\r");
//             result.put(Boolean.FALSE,sBuilder.toString());
//         } else {
//             String showPhysicalTop = IntStream.range(0, preNodeList.size()).mapToObj(index -> {
//                 PhysicalNode node = preNodeList.get(index);
//                 Integer deductResource = Optional.ofNullable(dq.poll()).orElse(0);
//                 Integer num = node.getNodeNum();
//                 // Integer remainVCPU = node.remainVCPU() - deductResource;
//                 // node.setRemainVCPU(remainVCPU);
//                 // return "{Node "+num+ "} {NF" + (index+1) + "("+deductResource+" vCPUs)}";
//                 return "{Node "+num+ "}";
//             }).collect(Collectors.joining(" -> "));
//             sBuilder.append(showPhysicalTop);
//             sBuilder.append("\n\r");
//             result.put(Boolean.TRUE,sBuilder.toString());
//         }
//         return result;
//     }

//     private void buildPrePath(Integer[] predecessor, int current, List<PhysicalNode> prePath,PhysicalNode[] physicalNodes) {
//         if (current == -1) {
//             return;
//         }
//         buildPrePath(predecessor, predecessor[current],prePath,physicalNodes);
//         prePath.add(physicalNodes[current]);
//     }

//     // Dijkstra's algorithm
//     private Integer shortestPathDelay(Integer initVertex,Integer vNFs, PriorityQueue<Integer> vNFsResource, Integer[] predecessor){
//         Graph graph = getInfrastructure();
//         Integer size = graph.V();
//         int INF = 0x3f3f3f3f;
//         int[] distance = new int[size];
//         Arrays.fill(distance, INF);
//         distance[initVertex] = 0;
//         PriorityQueue<int[]> pq = new PriorityQueue<int[]>(size, Comparator.comparingInt(o -> o[1]));
//         pq.add(new int[] {initVertex,0});
//         while (!pq.isEmpty()) {
//             int[] node = pq.poll();
//             Integer nodeNum = node[0];
//             Integer miniDelay = node[1];
//                 for (PhysicalNode adjPhysicalNode : graph.adj(nodeNum)) {
//                     Integer adjNodeNum = adjPhysicalNode.getNodeNum();
//                     Integer adjMiniDelay = adjPhysicalNode.toDelay + miniDelay;
//                     if (adjMiniDelay < distance[adjNodeNum]) {
//                         distance[adjNodeNum] = adjMiniDelay;
//                         predecessor[adjNodeNum] = nodeNum;
//                         pq.add(new int[]{adjNodeNum,adjMiniDelay});
//                     }
//                 }
//         }
//         return distance[size-1] == INF ? -1:distance[size-1];
//     }

//     public static void main(String[] args) {
//             SFCSimulator simulator = new SFCSimulator();
//             simulator.initInfrastructure();
//             simulator.showValidSFC(Arrays.asList(0,1,2),3);
//             simulator.showPhysicalNodeLinkDelay();
//             System.out.println();
//             simulator.showPhysicalNodeCapacityCPU();
//             System.out.println();
//             simulator.showPhysicalNodeRemainCPU();
//             System.out.println();
//             // real world
//             // List<Map<Boolean,String>> allSFCResult = simulator.simulateSFC(Arrays.asList(0,1,2));
//             // List<String> allStrings = allSFCResult.stream()
//             // .flatMap(map -> map.values().stream())
//             // .collect(Collectors.toList());
//             // for (String string : allStrings) {
//             //     System.out.println(string);
//             // }
//     //    String result =  simulator.simulateSFC(1,10,0,1,new Random());
//     //    System.out.println(result);
//         // Map<Boolean,List<String>> result = simulator.simulate3SFC();
//         // List<String> resultDesc = result.values().stream().findFirst().get();
//         // for (String desc : resultDesc) {
//         //     System.out.println(desc);
//         // }

//         // forShowAllValidSFCOutput();        
//         // SFC1(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay
//         // SFC2(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay
//         // SFC3(3 ~ 7 VNFs), VNFs(1 ~ 3 vCPUs), total 10 ~ 15 ms delay

        
//     }

    
// }