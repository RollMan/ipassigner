minikube status
if [ $? = 0 ]; then
  echo "minikube is still running."
  exit 1
fi

minikube start --vm-driver kvm2 --extra-config=apiserver.Admission.PluginNames=DefaultStorageClass
minikube ssh "sudo mkdir /exports"
minikube ssh "sudo mount -t nfs 192.168.122.166:/var/exports /exports"
