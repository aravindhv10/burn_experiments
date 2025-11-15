#include "stdio.h"
#include <iostream>
#include <vector>

#include <torch/torch.h>
#include <torch/csrc/inductor/aoti_package/model_package_loader.h>
extern "C" {
int torchmain() {
  printf("Working...\n");

    return 0;
}
}
