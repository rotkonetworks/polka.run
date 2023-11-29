use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container p-8 mx-auto">
        <h1 class="text-3xl font-bold text-center md:text-4xl lg:text-5xl">"PolkaVM: The Future of Virtual Machines"</h1>
        <div class="flex flex-col md:flex-row gap-6 mt-6">
        <div class="md:w-1/2">
        <p class="text-md leading-relaxed">
        "PolkaVM is a general-purpose, user-level virtual machine based on the RISC-V architecture, 
        designed for a range of applications from embedded systems to large-scale cloud computing."
        </p>
        <h2 class="text-xl mt-4 font-semibold">"Key Features"</h2>
        <ul class="list-disc pl-5">
        <li><strong>"Security"</strong>
        " - Utilizing sandboxing and a secure-by-design approach for enhanced protection."</li>
        <li><strong>"Determinism"</strong>
        " - Ensuring consistent and predictable performance, crucial for applications like smart contracts."</li>
        <li><strong>"Performance"</strong>
        " - Leveraging the efficiency of the RISC-V architecture for optimized execution and high-speed computations."</li>
        </ul>
        </div>
        <div class="md:w-1/2">
        <img class="w-full h-auto rounded-xl shadow-xl" src="/images/polkavm.png" alt="polkavm-hero"/>
        </div>
        </div>
        <div class="mt-6">
        <p class="text-md">
        "PolkaVM's combination of security, determinism, and performance positions it as a promising 
        lightweight virtual machine, making it ideal for a wide range of applications."
        </p>
        </div>
        </div>
    }
}
