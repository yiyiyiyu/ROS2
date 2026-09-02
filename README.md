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
- [3.动手安装ROS2](https://fishros.com/d2lros2/#/humble/chapt1/get_started/3.动手安装ROS2)
- [第 1 章：ROS1 核心概念回顾 | ros2_tutorial](https://zsc.github.io/ros2_tutorial/chapter1.html)



# 1. 架构



# 2. 工具



