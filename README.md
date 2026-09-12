# 版本

- Lyrical - Ubuntu Resolute Raccoon (26.04)
- Jazzy - Ubuntu Noble (24.04)
- Humble - Ubuntu Jammy (22.04)

# 下载

1. Set locale: 要求有一个可以支持 `UTF-8`的locale

   ```bash
   locale  # check for UTF-8
   
   sudo apt update && sudo apt install locales
   sudo locale-gen en_US en_US.UTF-8
   sudo update-locale LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8
   export LANG=en_US.UTF-8
   
   locale  # verify settings
   ```

2. Setup Sources：添加 Ros2 apt 的仓库到系统

   - 添加 `Ubuntu Universe repository`

     ```bash
     sudo apt install software-properties-common
     sudo add-apt-repository universe
     ```

   - 添加 Ros2 仓库

     ```bash
     sudo apt update && sudo apt install curl -y
     export ROS_APT_SOURCE_VERSION=$(curl -s https://api.github.com/repos/ros-infrastructure/ros-apt-source/releases/latest | grep -F "tag_name" | awk -F'"' '{print $4}')
     curl -L -o /tmp/ros2-apt-source.deb "https://github.com/ros-infrastructure/ros-apt-source/releases/download/${ROS_APT_SOURCE_VERSION}/ros2-apt-source_${ROS_APT_SOURCE_VERSION}.$(. /etc/os-release && echo ${UBUNTU_CODENAME:-${VERSION_CODENAME}})_all.deb"
     sudo dpkg -i /tmp/ros2-apt-source.deb
     ```

3. 下载Ros2

   ```bash
   sudo apt update
   sudo apt upgrade
   sudo apt install ros-humble-desktop # sudo apt install ros-humble-ros-base
   sudo apt install ros-dev-tools
   ```

4. source 

   ```bash
   source /opt/ros/humble/setup.bash
   ```

5. 卸载

   ```bash
   sudo apt remove '~nros-humble-*' && sudo apt autoremove 
   # 卸载仓库
   sudo apt remove ros2-apt-source
   sudo apt update
   sudo apt autoremove
   sudo apt upgrade # Consider upgrading for packages previously shadowed.
   ```

# 教程

- [Tutorials — ROS 2 Documentation: Humble documentation](https://docs.ros.org/en/humble/Tutorials.html)
- [动手学ROS2](https://fishros.com/d2lros2/#/)
- [ROS2 完全教程：从原理到实践 | ros2_tutorial](https://zsc.github.io/ros2_tutorial/)

# 1. 架构

## 1.1 ROS1

### 1.1.1 Master

**Master** 节点本质上是一个轻量级的名称服务器（Name Server），提供以下核心功能：

1. **名称注册与解析**：维护节点名称到网络地址的映射表, 维护了整个 ROS 计算图的“通讯录”.
   - **工作机制**：当一个 ROS 节点（Node）启动时，它会向 Master 报告自己的**节点名称**、**网络地址（URI）**，以及它要**发布（Publish）** 或**订阅（Subscribe）** 的话题（Topic）名称。Master 会将所有信息记录在内存中。
2. **服务发现**：帮助节点之间建立点对点连接
   - **工作流程（以话题通信为例）**：
     1. **发布者（Publisher）上线**：节点 A 启动，向 Master 注册，声明它将发布名为 `/camera` 的话题。
     2. **订阅者（Subscriber）上线**：节点 B 启动，向 Master 查询，表示它想订阅 `/camera` 话题。
     3. **Master“牵线”**：Master 发现 A 和 B 的需求匹配，于是将 A 的网络地址（IP 和端口）告诉 B。
     4. **节点“直连”**：节点 B 获得地址后，**直接**与节点 A 建立 TCP 连接。此后，所有的图像数据都通过这个直连链路传输，完全绕开 Master。
3. **参数服务器**：存储和分发全局配置参数
   - **工作机制**：Master 内部维护了一个全局的、基于键值对（Key-Value）的字典（Dictionary）。任何节点都可以通过 `rosparam` 等工具或 API，向这个“白板”**设置（set）**、**获取（get）** 或**删除（delete）** 参数。
   - **主要作用**：它主要用于存储系统运行时的配置参数，例如 PID 控制器的增益、相机分辨率等。这样做的好处是配置与代码分离，便于集中管理和动态调整。

```
     +----------------+
     |   ROS Master   |
     |   (roscore)    |
     +-------+--------+
             |
     +-------+--------+
     |  Name Service  |
     |   Registry     |
     +----------------+
            / | \
           /  |  \
    +-----+   |   +-----+
    |Node1|   |   |Node2|
    +-----+   |   +-----+
              |
          +-------+
          |Node3  |
          +-------+
```

**总结**：

- **“轻量级的名称服务器”** 准确描述了它**不转发数据**，只处理元数据的特性。
- **“名称注册与解析”** 是其作为“电话总机”的看家本领。
- **“服务发现”** 是实现高效 **P2P 通信** 的关键“牵线”步骤。
- **“参数服务器”** 是其提供的、用于**集中存储配置**的附加功能。

> 这三大功能共同构成了 ROS 1 计算图的核心枢纽，但其集中式的设计也带来了单点故障等架构上的局限性。

### 1.1.2 XMLRPC 协议与通信流程

ROS1 使用 XMLRPC 作为**节点**与 **Master**之间的通信协议。

**为什么是 XML-RPC？**

ROS 诞生于 2007 年左右，彼时分布式通信的选择有限。选择 XML-RPC 而非现代流行的 gRPC 或 RESTful，基于以下考量：

- **极致的轻量级**：XML-RPC 基于 HTTP 协议，传输的是纯文本的 XML 格式。它不需要复杂的序列化（如 Protobuf），库文件极小，非常契合机器人嵌入式硬件的有限资源。

- **绝佳的语言无关性**：几乎所有编程语言（C++、Python、Java、Lisp）都对 XML-RPC 有原生支持。ROS 1 希望节点可以用不同语言编写，XML-RPC 是当时实现这一目标最便捷的“最大公约数”。

- **仅用于控制信令**：设计者非常清楚，XML 文本格式解析慢、占用带宽大，所以**严格限定 XML-RPC 只传输“元数据”（名字、地址、参数），绝不传输传感器点云、图像等“大数据”**。

  

**节点启动与注册流程**：

1. 节点启动时，通过 `ROS_MASTER_URI` 环境变量(比如 “http://192.168.1.100:11311”)找到 Master

2. 当节点启动时，它会在自己的内存中初始化，但**不会**主动去 Master 注册“我是一个节点”。它只在需要对外提供功能时，才去注册：

   - 如果是发布者（Publisher），调用 **`registerPublisher()`**
   - 如果是订阅者（Subscriber），调用 **`registerSubscriber()`**
   - 如果是服务端（Service），调用 **`registerService()`**
   - Master 的注册表里存的是 **（话题名 ↔ 节点 URI）** 的映射，而不是单独的节点列表。

3. **ROS 1 的 Master 不会给节点分配任何全局递增的数值型 ID（如 UUID）**。节点的“唯一标识”就是它在启动时通过 `ros::init()` 传入的 **节点名称（Node Name）**，比如 `/camera_node`。

   > **隐藏的“抢注”机制（容易被忽视）**：如果两个节点起了一模一样的名字（例如都叫 `/camera_node`），Master 不会给第二个节点分配一个新 ID，而是会**强制踢掉（Shutdown）第一个节点**，让第二个节点顶替上去。这就是 ROS 1 著名的 **“后发先至（抢注）”** 策略。所以，所谓的“注册确认”仅仅是 Master 回复一个 `1`（成功），并没有 ID 生成。

4. 节点注册自己提供的话题/服务到 Master。

   - 当调用 `registerPublisher` 时，节点向 Master 提交的数据包包含：
     1. **话题名称**（如 `/cmd_vel`）
     2. **节点的 XML-RPC 地址**（即该节点内置的一个小型 XML-RPC 服务器的 URI，用于接收其他节点的反向连接请求）
     3. **消息类型**（如 `geometry_msgs/Twist`）
     4. **消息 MD5 校验和**（用于确保发布者和订阅者编译的是完全相同的消息定义）
   - **关键作用**：当订阅者查询时，Master 会进行 **MD5 严格匹配**。如果 MD5 对不上，Master 依然会返回发布者地址，但订阅者收到后会发现类型不匹配并报错中断，**这是 ROS 1 编译时依赖强耦合的根源**。

5. 订阅者的“反向注册”

   - 订阅者通过 Master 拿到发布者的 XML-RPC URI 后，**订阅者会作为客户端，主动去连接发布者的 XML-RPC 服务器**。
   - 订阅者调用发布者提供的 **`requestTopic()`** 方法。
   - 发布者此时会动态分配一个**空闲的 TCP 端口**，通过 XML-RPC 返回给订阅者。
   - 订阅者拿到这个 TCP 端口后，**断开 XML-RPC 连接**，转而建立 **TCPROS（二进制数据流）** 的长连接。

**话题订阅建立流程**：

```
发布者节点                Master                订阅者节点
    |                      |                      |
    |--registerPublisher-->|                      |
    |                      |<--registerSubscriber-|
    |                      |                      |
    |                      |--publisherUpdate---->|
    |<-----------------requestTopic---------------|
    |------------------TCPROS连接----------------->|
```

> XML-RPC 下的完整“三次握手”流程（工作机制）

在 ROS 1 中，Master 只是一个 XML-RPC 服务器（Server），每个节点同时也是 XML-RPC 客户端（Client）。**节点与 Master 的交互全是短连接的 HTTP 请求**，典型流程如下：

1. **注册（Publisher）**：节点 A 通过 XML-RPC 调用 Master 的 `registerPublisher` 接口，告诉 Master：“我要发 `/camera` 话题，我的 IP 是 192.168.1.1:1234”。
2. **查询（Subscriber）**：节点 B 通过 XML-RPC 调用 Master 的 `registerSubscriber` 接口，询问：“谁在发 `/camera` 话题？”
3. **解析（返回地址）**：Master 通过 XML-RPC 响应，把节点 A 的 URI（192.168.1.1:1234）返回给节点 B。

**关键转折点**：拿到地址后，节点 B 会**直接通过 XML-RPC 去请求节点 A 的 TCP 服务器端口**，协商后续数据传输的端口号。协商成功后，**节点 A 和 B 之间建立 TCPROS（基于 TCP 的二进制流）连接**，从此所有图像、激光数据走二进制通道，完全踢开 Master 和 XML-RPC。

这个过程的关键点：

- Master 只负责”牵线搭桥”，不参与数据传输
- 节点之间建立直接的 TCPROS 连接传输数据
- 这种设计降低了 Master 负载，但也引入了单点故障

### 1.1.3 TCPROS协议

1. 第一阶段：连接建立（Connection Header）—— 纯文本键值对

   当订阅者（Subscriber）通过 XML-RPC 拿到发布者（Publisher）的 IP 和端口后，会创建一个 TCP 连接。**在这个连接发送任何二进制数据之前，必须交换一次文本头（Header）。**

   - **帧结构 1：请求头（Client -> Server）**

     | 字节偏移 | 字段       | 类型              | 值示例（ASCII）   | 说明                                    |
     | :------- | :--------- | :---------------- | :---------------- | :-------------------------------------- |
     | 0 - 3    | Header长度 | `uint32_t` (小端) | `0x00000038` (56) | 后续 Header 文本的字节长度              |
     | 4 - N    | Header数据 | `char[]`          | 见下方            | 由 `key=value` 组成，用 `\0`(0x00) 分隔 |

     **Header 数据内容（必须包含的键值对）：**

     ```text
     callerid=my_subscriber_node\0
     topic=/chatter\0
     md5sum=992ce8a1687cec8c8bd883ec7395c2fb\0
     type=std_msgs/String\0
     tcp_nodelay=1\0
     ```

     > **注意**：字符串末尾没有换行符，全是 `\0` 分隔。`tcp_nodelay=1` 建议带上，用于禁用 Nagle 算法。

   - **帧结构 2：响应头（Server -> Client）**

     发布者收到请求后，校验 MD5 和类型，回复响应头。格式完全相同：

     | 字节偏移 | 字段       | 类型              | 说明                              |
     | :------- | :--------- | :---------------- | :-------------------------------- |
     | 0 - 3    | Header长度 | `uint32_t` (小端) | 通常小于 256 字节                 |
     | 4 - N    | Header数据 | `char[]`          | 包含 `md5sum`、`type`、`callerid` |

     **响应后，连接正式进入数据传输阶段**。如果 MD5 校验失败，发布者会在响应后直接发送一个 `-1` 长度的包（特殊信号）并主动关闭连接。

2. 第二阶段：数据传输（Data Layer）—— 二进制序列化

   握手结束后，**发布者开始向 TCP 连接中持续写入数据帧**。

   **帧结构（每一条消息）：**

   | 字节偏移           | 字段名           | 类型          | 字节数     | 值域                        | 详细作用                                           |
   | :----------------- | :--------------- | :------------ | :--------- | :-------------------------- | :------------------------------------------------- |
   | **0 - 3**          | **Payload 长度** | **uint32_t**  | **4 字节** | **0x00000000 ~ 0xFFFFFFFF** | **小端序**。表示后面紧跟的序列化数据占了几个字节。 |
   | **4 - (4+Length)** | **序列化数据**   | **uint8_t[]** | **可变**   | 二进制裸数据                | 具体的 ROS 消息内容，按内存顺序平铺。              |

3. 举个实际的十六进制（Hex）流例子

   假设你要发送 `std_msgs/String`，内容为 `"Hi"`：

   - ​	**计算负载长度**：字符串长度前缀（4字节）+ 实际字符（2字节）= 6字节。写入 `uint32_t` 小端：`0x06 0x00 0x00 00`。

   - **序列化消息体（Msg）**：

     先写长度 `2`：`0x02 0x00 0x00 0x00`

     再写字符 `'H'` 和 `'i'`：`0x48 0x69`

   - **所以完整的 TCP 字节流为：**
     `[06 00 00 00] [02 00 00 00] [48 69]`

> ### TCPROS 序列化格式的“隐形陷阱”（“比 Protobuf 简单?”）
>
> - **致命差异（反直觉）**：Protobuf 是**有 Schema（模式）** 的自描述格式，接收方即使没有编译 .proto 文件也能解析部分字段。但 **ROS 1 的序列化格式是完全“无头（Headerless）”的**，它只按顺序紧密排列二进制原始数据（Primitive Types）。
> - **这意味着什么？** 订阅者**必须**提前知道消息的确切结构和字段顺序。如果发布者编译的 `geometry_msgs/Twist` 和订阅者编译的版本 MD5 校验和不一致（哪怕只差一个字段名），TCP 连接虽然能建立，但订阅者反序列化时会**直接内存错位**，导致程序崩溃或读出天文数字。这就是为什么 ROS 1 编译时强依赖，且不能热更新消息定义。

**性能特征分析**：

- 延迟：局域网 < 1ms，取决于消息大小和网络状况
- 吞吐量：可达网络带宽的 80-90%（大消息）
- CPU 开销：序列化/反序列化约占 5-15%（取决于消息复杂度）

### 1.1.4 多机通信配置

在多机环境下部署 ROS1 需要careful配置：

```bash
# 机器 A (Master 所在)
export ROS_MASTER_URI=http://192.168.1.100:11311
export ROS_IP=192.168.1.100

# 机器 B (Worker 节点)
export ROS_MASTER_URI=http://192.168.1.100:11311
export ROS_IP=192.168.1.101
export ROS_HOSTNAME=worker-robot  # 可选，用于 DNS 解析
```

**网络配置检查清单**：

1. 所有机器时钟同步（NTP）
2. 防火墙开放必要端口（11311 for Master, 随机端口 for nodes）
3. 主机名解析正确（/etc/hosts 或 DNS）
4. 网络延迟 < 10ms（局域网环境）

### 1.1.5 通信机制

- ### 话题（Topics）：发布-订阅模式

  **消息传输特征**：

  - **异步通信**：发布者不等待订阅者接收

  - **多对多通信**：多个发布者和订阅者可以共享同一话题。

    > - **工程现实**：**在 TCPROS 层面，不存在“共享”总线。**
    >   如果有 N 个发布者和 M 个订阅者，Master 牵线后，底层会建立 **N × M 个独立的 TCP 长连接**。这意味着：
    >   - **资源消耗**：如果有 3 个相机节点（发布者）和 4 个视觉节点（订阅者），底层会建立 12 个 TCP 连接。端口资源和服务端 `select/poll` 的句柄数会线性增长。
    >   - **广播风暴风险**：如果某个发布者发送 1MB 的点云数据，这条数据会在 TCP 层被复制 M 份，占用 M 倍的网络带宽（而非共享内存）。

  - **无应答机制**：发布者不知道消息是否被接收

    

  **队列管理策略**：发布者和订阅者都会维护各自的队列

  - **发布者队列：管理消息发送**

    发布者队列是一个**出站（outgoing）消息队列**，主要负责：

    - **缓冲待发消息**：当`publish()`被调用时，消息会先进入这个队列等待发送。
    - **应对网络波动**：在网络不稳定或订阅者处理慢时，队列能暂存消息，避免丢失。
    - **控制内存使用**：通过`queue_size`参数限制队列长度，防止无限制增长导致内存溢出。

    当队列满了之后，再有新消息进入，最旧的消息会被丢弃。

    ```python
    # 发布者队列大小设置
    pub = rospy.Publisher('topic', MessageType, queue_size=10)
    # queue_size 影响：
    # - 太小：高频发布时可能丢失消息
    # - 太大：占用内存，增加延迟
    ```
    - **roscpp (C++) 的发布者**：当 TCP 发送缓冲区满（订阅者接收太慢）且发布队列 `queue_size` 填满时，新来的消息会**覆盖（Overwrite）队列中最旧的消息**。即：**丢弃老数据，保留新数据**（适合传感器流，雷达数据旧不如新）。
    - **rospy (Python) 的发布者（高风险）**：**在 ROS Kinetic 及更早版本中，若 `queue_size=0`，队列大小被视为无限**！如果订阅者卡住，Python 进程内存会疯狂增长直至 OOM（内存溢出）被系统杀死。在 Noetic 中，`queue_size=0` 被修正为“只存 1 条”。

  - **订阅者的回调队列**：入站（incoming）消息队列

    ```python
    rospy.Subscriber('chatter', String, callback, queue_size=10)
    ```

    - **暂存待处理消息**：接收到的消息会先存放在这里，等待回调函数（Callback）处理。
    - **解耦接收与处理**：即使回调函数处理较慢，消息也能先被接收并存放在队列中，避免丢失。
    - **控制内存使用**：同样通过`queue_size`参数限制队列长度，防止内存溢出。

    当队列满了之后，再有新消息到达，最旧的消息会被丢弃。

- ### 服务（Services）：请求-响应模式

  服务提供同步的请求-响应通信模式，适合需要确定性结果的场景。

  **服务调用流程**：

  ```
  客户端                    服务器
    |                         |
    |---请求（Request）------>|
    |                         |处理请求
    |<---响应（Response）------|
    |                         |
  ```

  **关键设计决策**：

  1. **同步阻塞**：客户端等待服务器响应
     - 默认超时时间是无穷大（`None`），意味着如果服务端没响应，客户端会**永久挂起**。
     - **生产环境必加超时**：建议在 Proxy 构造或 `call()` 时设置 `timeout`（如 `rospy.ServiceProxy('name', Srv, timeout=10)`），否则网络闪断或服务端死锁会导致整个 Node 卡死。
  2. **单次连接**：每次调用建立新的 TCP 连接
  3. **无状态**：服务器不维护客户端状态

  **性能考量**：

  ```
  服务调用开销 = 连接建立时间 + 请求传输 + 处理时间 + 响应传输
  ```

  **持久连接优化**：

  ```python
  # 使用持久连接减少开销
  from rospy import ServiceProxy
  service = ServiceProxy('service_name', ServiceType, persistent=True)
  # 重用 TCP 连接，减少握手开销
  ```

  **但需要特别注意**：如果服务端（Server）节点崩溃重启，持久连接会变为“僵尸连接”。此时调用 `service.call()` 会抛出 `ServiceException`，必须捕获异常并重新创建 `ServiceProxy` 才能恢复。

- ### 动作（Actions）：带反馈的异步任务

  适合长时间运行的任务

  **动作协议的五个组成部分**：

  1. **Goal**：任务目标
  2. **Result**：最终结果
  3. **Feedback**：执行过程中的反馈
  4. **Status**：任务状态（pending/active/succeeded/aborted）
  5. **Cancel**：取消机制

  ```
  动作内部实现 = 5个话题 + 状态机管理
             /action_name/goal        (目标发送)
             /action_name/cancel      (取消请求)
             /action_name/status      (状态更新)
             /action_name/feedback    (进度反馈)
             /action_name/result      (最终结果)
  ```

  **状态机转换图**：

  ```
          [PENDING]
              |
              v
          [ACTIVE] <---> [PREEMPTING]
           /    \              |
          v      v             v
     [SUCCEEDED] [ABORTED] [PREEMPTED]
  ```

  **设计模式应用场景**：

  - **导航任务**：发送目标点，接收路径执行反馈
  - **机械臂控制**：执行轨迹，监控执行进度
  - **感知处理**：长时间的图像处理或 SLAM 建图

### 1.1.6 Catkin

Catkin 是 ROS1 的构建系统，基于 CMake 扩展而来，解决了大规模机器人软件的构建挑战。

**核心设计目标**：

1. **包管理**：支持细粒度的功能包组织
2. **依赖管理**：自动处理包之间的依赖关系
3. **并行构建**：充分利用多核 CPU
4. **跨平台**：支持 Linux、macOS（部分）

**工作空间结构**

```
catkin_ws/
├── src/               # 源代码目录
│   ├── package1/
│   │   ├── CMakeLists.txt
│   │   ├── package.xml
│   │   ├── src/
│   │   └── include/
│   └── package2/
├── build/             # 构建中间文件
│   └── [CMake 生成的构建文件]
├── devel/             # 开发空间
│   ├── setup.bash     # 环境配置脚本
│   ├── lib/           # 编译的库文件
│   └── share/         # 资源文件
└── install/           # 安装空间（可选）
    └── [发布版本文件]
```

**CMakeLists.txt 深度解析**

```cmake
cmake_minimum_required(VERSION 3.0.2)
project(my_robot_package)

# 查找 catkin 和依赖包
find_package(catkin REQUIRED COMPONENTS
  roscpp
  std_msgs
  sensor_msgs
  geometry_msgs
)

# 声明 catkin 包
catkin_package(
  INCLUDE_DIRS include
  LIBRARIES ${PROJECT_NAME}
  CATKIN_DEPENDS roscpp std_msgs
  DEPENDS eigen3  # 系统依赖
)

# 包含目录
include_directories(
  include
  ${catkin_INCLUDE_DIRS}
)

# 编译库
add_library(${PROJECT_NAME}
  src/algorithm.cpp
)

# 编译可执行文件
add_executable(robot_node src/main.cpp)
target_link_libraries(robot_node
  ${PROJECT_NAME}
  ${catkin_LIBRARIES}
)

# 安装规则
install(TARGETS robot_node
  RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)
```

**包依赖管理**

**package.xml 结构**：

```xml
<?xml version="1.0"?>
<package format="2">
  <name>my_robot_package</name>
  <version>1.0.0</version>
  <description>机器人控制包</description>
  
  <maintainer email="dev@robot.com">Developer</maintainer>
  <license>MIT</license>
  
  <!-- 构建依赖 -->
  <buildtool_depend>catkin</buildtool_depend>
  <build_depend>roscpp</build_depend>
  
  <!-- 运行依赖 -->
  <exec_depend>roscpp</exec_depend>
  <exec_depend>rospy</exec_depend>
  
  <!-- 测试依赖 -->
  <test_depend>rostest</test_depend>
</package>
```

**依赖解析算法**：

1. 拓扑排序确定构建顺序
2. 检测循环依赖
3. 并行构建无依赖关系的包

**构建优化技巧**

**1. 并行构建加速**：

```
# 使用所有 CPU 核心
catkin_make -j$(nproc)

# 或使用 catkin_tools（推荐）
catkin build --jobs $(nproc)
```

**2. 增量构建优化**：

```
# 只构建修改的包
catkin build --this

# 构建指定包及其依赖
catkin build package_name --deps
```

**3. ccache 加速重复编译**：

```
# 安装 ccache
sudo apt-get install ccache

# 配置 catkin 使用 ccache
export CC="ccache gcc"
export CXX="ccache g++"
```

### 1.1.7 参数服务器

ROS1 的参数服务器是一个中心化的配置存储系统，运行在 Master 节点上。它使用层次化的命名空间存储键值对。

**参数类型支持**：

- 基本类型：bool, int, double, string
- 复合类型：list, dict（嵌套结构）
- 二进制数据：base64 编码的二进制 blob

**命名空间层次结构**：

```
/
├── robot_name              # 全局参数
├── /navigation/
│   ├── max_velocity        # 导航模块参数
│   ├── planner/
│   │   ├── algorithm       # 规划器配置
│   │   └── resolution
│   └── controller/
│       └── gains           # 控制器参数
└── /perception/
    ├── camera/
    │   └── fps
    └── lidar/
        └── range
```

**参数操作 API**

- **参数读写操作**：

  ```python
  # Python API
  import rospy
  
  # 读取参数
  max_vel = rospy.get_param('/navigation/max_velocity', 1.0)  # 带默认值
  params = rospy.get_param('/navigation/')  # 获取整个命名空间
  
  # 写入参数
  rospy.set_param('/navigation/max_velocity', 2.0)
  
  # 删除参数
  rospy.delete_param('/navigation/obsolete_param')
  
  # 检查参数存在
  if rospy.has_param('/navigation/max_velocity'):
      # 参数存在
      pass
  
  # C++ API
  ros::NodeHandle nh;
  double max_vel;
  nh.getParam("/navigation/max_velocity", max_vel);
  nh.setParam("/navigation/max_velocity", 2.0);
  ```

- **私有参数与相对命名**

  ```python
  # 私有参数（节点命名空间）
  rospy.init_node('my_node')
  # 参数实际路径：/my_node/param_name
  private_param = rospy.get_param('~param_name')
  
  # 相对参数（当前命名空间）
  # 如果当前命名空间是 /robot1/
  relative_param = rospy.get_param('sensor/range')
  # 实际路径：/robot1/sensor/range
  ```

**动态重配置（Dynamic Reconfigure）**

**配置文件定义（.cfg）**：

```c++
#!/usr/bin/env python
from dynamic_reconfigure.parameter_generator_catkin import *

gen = ParameterGenerator()

# 添加参数：名称、类型、级别、描述、默认值、最小值、最大值
gen.add("max_velocity", double_t, 0, 
        "Maximum velocity", 1.0, 0.0, 5.0)
gen.add("enable_obstacle_avoidance", bool_t, 0,
        "Enable obstacle avoidance", True)

# 枚举类型
algorithm_enum = gen.enum([
    gen.const("DWA", int_t, 0, "Dynamic Window Approach"),
    gen.const("TEB", int_t, 1, "Timed Elastic Band"),
    gen.const("MPC", int_t, 2, "Model Predictive Control")
], "Planning algorithm selection")

gen.add("algorithm", int_t, 0, 
        "Path planning algorithm", 0, 0, 2, 
        edit_method=algorithm_enum)

exit(gen.generate("my_package", "my_node", "MyConfig"))
```

**节点实现动态重配置**：

```c++
#include <dynamic_reconfigure/server.h>
#include <my_package/MyConfigConfig.h>

class MyNode {
private:
    dynamic_reconfigure::Server<my_package::MyConfigConfig> server_;
    
    void configCallback(my_package::MyConfigConfig &config, uint32_t level) {
        // 更新内部参数
        max_velocity_ = config.max_velocity;
        use_obstacle_avoidance_ = config.enable_obstacle_avoidance;
        
        // 级别检查（哪些参数改变了）
        if (level & 0x1) {
            // 速度参数改变
            updateVelocityController();
        }
    }
    
public:
    MyNode() {
        // 设置回调
        server_.setCallback(
            boost::bind(&MyNode::configCallback, this, _1, _2));
    }
};
```



## 1.2 ROS2

### 1.2.1 架构

![image-20220602204152352](ROS2架构图.png)

- DDS实现层：数据分发服务（Data Distribution Service, DDS）。

  DDS 的核心概念模型：

  ```
  ┌─────────────────────────────────────────────────────┐
  │                   DDS Domain                        │
  │  ┌──────────────┐              ┌──────────────┐    │
  │  │ Domain       │              │ Domain       │    │
  │  │ Participant  │◄────────────►│ Participant  │    │
  │  │              │     RTPS      │              │    │
  │  │ ┌──────────┐ │              │ ┌──────────┐ │    │
  │  │ │Publisher │ │              │ │Subscriber│ │    │
  │  │ │          │ │              │ │          │ │    │
  │  │ │┌────────┐│ │              │ │┌────────┐│ │    │
  │  │ ││DataWriter│─────Topic────►│ ││DataReader│ │    │
  │  │ │└────────┘│ │              │ │└────────┘│ │    │
  │  │ └──────────┘ │              │ └──────────┘ │    │
  │  └──────────────┘              └──────────────┘    │
  └─────────────────────────────────────────────────────┘
  ```

  1. Transport Layer：直接调用操作系统的**Socket API**（UDP/IP协议栈）
     - **多播与单播管理**：它负责在局域网内发送组播（Multicast）来寻找其他节点，同时建立单播（Unicast）通道来传输实际的大数据。
     - **共享内存（SHM）**：如果两个节点在同一台电脑上，Fast DDS会直接绕过网卡，通过**共享内存**传递数据（零拷贝），这一块逻辑完全由它自己掌控，RMW压根不知道数据是走网线还是走内存。
  2. Discovery & Topology：维护着一张巨大的**内置主题（Built-in Topics）**列表。
     - 它会在后台不停地收发RTPS（实时发布-订阅协议）的发现报文。
     - **具体工作**：它会记录当前网络里“谁发布了什么Topic”、“谁订阅了什么Topic”、“谁的IP地址是多少、端口号是多少”。这个匹配过程（称为**端点匹配 Endpoint Matching**）完全在实现层内部完成，匹配成功后，它才会建立两个节点间的“虚连接”。
  3. Serialization Engine：当调用`publish`时，数据交到实现层手上，它会立刻启动**CDR（通用数据表示）引擎**。
     - 它将C++/Python的结构体数据，**按照固定的字节序（大端/小端）打平成二进制流**。
     - **碎片化处理**：如果打平后的数据大于一个网络包（如超过1472字节），实现层会主动将其**切成多个RTPS子报文（Submessage）**，并在包头写上序号，方便接收端重组。ROS上层传下来的只是指针，它才是真正“动手拆解”数据的那个人。

  4. Reliability & QoS:发数据，它还**承诺送达质量**
     - **消息重传（NACK/ACK机制）**：当你设置QoS为`RELIABLE`时，发送端会把发过的数据存进**本地历史缓存（Writer History Cache）**。如果接收端没收到，会发回一个NACK（否定应答）信号，实现层收到后就会从历史缓存中**取出那包数据重新发送**，直到对方确认（ACK）。
     - **心跳（Heartbeat）**：即使没有数据发送，实现层内部的后台线程也会定时向对端发送空包，以确认对方“还活着”，维持连接的活性。
  5. Threading & Scheduling:实现层一旦被加载进ROS节点进程，就会**创建属于它自己的后台线程池**：
     - **接收线程**：一个专用线程一直在`recv()`系统调用上阻塞，监听网口涌入的数据包。
     - **事件线程**：处理定时器，比如重传计时器、心跳计时器。
     - **注意**：这些线程由实现层直接调度，不受ROS节点`spin()`的控制。即使你的ROS主循环卡住了，实现层的网络接收线程依然在工作，把数据存入缓存（DDS的`DataReader`缓存）里等你来取。

- 协议实时发布订阅协议（Real-Time Publish-Subscribe, RTPS）

  **RTPS 实体模型**

  - **Participant**：代表通信域中的一个应用实例
  - **Endpoint**：通信端点，包括 Writer 和 Reader
  - **GUID**：全局唯一标识符，格式为 `{prefix, entityId}`

  **发现协议（Discovery Protocol）**

  RTPS 使用两阶段发现机制

  1. 简单参与者发现协议（SPDP）
     - 使用多播定期广播 Participant 信息
     - 默认多播地址：`239.255.0.1:7400` (IPv4)
     - 发现消息包含：GUID、位置器（Locator）、QoS 等
  2. 简单端点发现协议（SEDP）
     - 在已发现的 Participant 间交换 Endpoint 信息
     - 使用可靠单播通信
     - 交换 Topic 名称、类型信息、QoS 配置

  **可靠性机制**：

  RTPS 提供多种可靠性保证

  ```
  心跳机制（Heartbeat）：
  Writer ──HB(seq_min, seq_max)──► Reader
        ◄──────ACK/NACK───────────
        ────────DATA──────────────►
  
  其中：
  - HB: 心跳消息，包含序列号范围
  - ACK: 确认接收
  - NACK: 请求重传
  ```

- DDS抽象层：RMW（ROS MiddleWare）

  ROS2 通过 RMW（ROS MiddleWare）接口抽象不同的 DDS 实现：

  ```c++
  // RMW 接口示例
  typedef struct rmw_node_t {
    const char * name;
    const char * namespace_;
    rmw_context_t * context;
    rmw_node_impl_t * impl;
  } rmw_node_t;
  
  // 创建节点的统一接口
  rmw_node_t * rmw_create_node(
    rmw_context_t * context,
    const char * name,
    const char * namespace_);
  ```

  RMW 层次结构：

  ```
  ┌─────────────┐
  │  ROS2 API   │  <- rclcpp/rclpy
  ├─────────────┤
  │     RCL     │  <- ROS Client Library
  ├─────────────┤
  │     RMW     │  <- 抽象接口层
  ├─────────────┤
  │  DDS Impl   │  <- Fast DDS/Cyclone等
  ├─────────────┤
  │   Network   │  <- UDP/共享内存
  └─────────────┘
  ```

- ROS2 客户端库 RCL

- 应用层

### 1.2.2 实时性



### 1.2.3 内存管理

**1. 预分配内存池**：

```C++
// 使用自定义分配器避免运行时分配
template<typename T>
class PoolAllocator {
  private:
    std::array<T, POOL_SIZE> pool_;
    std::bitset<POOL_SIZE> used_;
    
  public:
    T* allocate(size_t n) {
        // O(1) 时间复杂度的分配
        for(size_t i = 0; i < POOL_SIZE; ++i) {
            if(!used_[i]) {
                used_[i] = true;
                return &pool_[i];
            }
        }
        throw std::bad_alloc();
    }
};
```

**2. 零拷贝通信**：

ROS2 支持通过共享内存实现零拷贝：

```
进程 A                     共享内存                    进程 B
┌──────┐                 ┌─────────┐                ┌──────┐
│Writer│───write ptr────►│ Message │◄───read ptr────│Reader│
└──────┘                 └─────────┘                └──────┘
         无需复制，直接访问同一内存区域
```

### 1.2.4 调度策略与优先级



### 1.2.5 时间抖动分析



# 2. 工具



