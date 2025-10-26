class ServiceMonitor:
    """Provides service checks for a single service"""

    service_name: str
    port: int
    is_alive: bool

    def __init__(self, service_name: str, port: int) -> None:
        """Initializes the monitor for a specific service

        Args:
            service_name (str): The name of the service
            port (int): The port to use for checks
        """

        print(f"Intializing monitor for service {service_name} on port {port}")
        self.service_name = service_name
        self.port = port
        self.is_alive = False


def main():
    # NOTE: postional arguments
    nginx_monitor: ServiceMonitor = ServiceMonitor("nginx", 80) 

    print(f"nginx_monitor service_name: {nginx_monitor.service_name}")
    print(f"nginx_monitor port: {nginx_monitor.port}")

    # How to view the type of an instance 
    # print(type(nginx_monitor))
    # <class '__main__.ServiceMonitor'>

    # How to check if an instance has been created from a specific class
    # isinstance(instance_name, ClassNam)
    
    # print(isinstance(nginx_monitor, ServiceMonitor))
    # True

    # NOTE: kwargs (keyword arguments)
    # Keyword arguments explicitly name the parameter in the function call.
    redis_monitor = ServiceMonitor(service_name="redis", port=6379)

    print(f"redis_monitor service_name: {redis_monitor.service_name}")
    print(f"redis_monitor port: {redis_monitor.port}")

if __name__ == "__main__":
    main()
