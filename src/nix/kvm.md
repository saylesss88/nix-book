# Running NixOS as a VM for added isolation

- [Redhat virtualization administration](https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/6/pdf/virtualization_administration_guide/virtualization-administration-guide.pdf)

## Secureblue

I recommend using Secureblue as your host, Fedora is one of the few distros that
enable SELinux at a system level by default and Secureblue takes many more
hardening steps. Fedora also uses sVirt, a technolocy that integrates SELinux
and virtualization. sVirt applies Mandatory Access Control (MAC) to improve
security when using guest virtual machines.
