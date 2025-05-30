.. _llm_router:

LLM Routing
==============================================================


LLM Router systems are designed to direct incoming queries to the most appropriate model or processing pipeline. The core challenge stems from the complexity of natural language, the wide spectrum of incoming requests, and the ambiguity of task definitions. 
Traditional **performance-based** routing focus on selecting the language model most likely to deliver high-quality responses for each query while staying within cost and latency constraints.
For **preference-based routing**, instead of predicting raw model quality, it routes incoming query to the option that best aligns with human preferences provided by end users in terms of a route usage config which defines the routing logic and selects the model for each task category.


Arch Router
-----------
 is a powerful feature in Arch that allows your application to dynamically change the response LLM based on user prompts.
Arch-Router matches user prompts to high-level task categories specified by developers (e.g., FAQ answer, creative writing, code generation), and routes each query to the corresponding model or pipeline. 
This developer-first approach makes routing decisions more transparent and adaptable, reflecting the practical definitions of quality that matter in production environments.


Routing Workflow
-------------------------

#. **Prompt Parsing**

    When a user submits a prompt, Arch analyzes it to determine the most relevant route matching to the context. 

#. **Response Handling**

    After the route has been detected, the Arch Router selects the appropriate model or pipeline to handle the request. 

Arch-Router-1.5B
-------------------------
The `Arch-Router-1.5B <https://huggingface.co/katanemo/Arch-Router-1.5B>`_ is a routing architecture that aligns model selection with developer-defined task descriptors.
In summary, the Arch-Router collection demonstrates:

- **State-of-the-art performance** in preference routing - model performs equal or better than all close-source model in routing task (white paper incoming soon)
- **High generalization**, the model is able to understand vast domains, even in ambiguous or subjective routes like complex and simple.
- Optimized **low-latency, high-throughput performance**, making it suitable for real-time, production environments.


Writing Route Config
-----------------------------

To implement the Arch Router, you need to define a prompt target configuration that specifies the routing model and the LLM providers. This configuration will allow Arch Gateway to route incoming prompts to the appropriate model based on the defined routes.

There is a sample configuration below that demonstrates how to set up a prompt target for the Arch Router:

- Define the routing model in the `routing` section. You can use the `archgw-v1-router-model` as the katanemo routing model or any other routing model you prefer.
- Define the listeners in the `listeners` section. This is where you specify the address and port for incoming traffic, as well as the message format (e.g., OpenAI).
- Define the LLM providers in the `llm_providers` section. This is where you specify the routing model, the OpenAI models, and any other models you want to use for specific tasks (e.g., code generation, code understanding).
- Make sure you define a model for default usage, such as `gpt-4o`, which will be used when no specific route is matched for an user prompt.


.. code-block:: yaml
    :caption: Route Config Example


    routing:
    model: archgw-v1-router-model

    listeners:
    egress_traffic:
        address: 0.0.0.0
        port: 12000
        message_format: openai
        timeout: 30s

    llm_providers:
    - name: archgw-v1-router-model
        provider_interface: openai
        model: katanemo/Arch-Router-1.5B
        base_url: ...

    - name: gpt-4o-mini
        provider_interface: openai
        access_key: $OPENAI_API_KEY
        model: gpt-4o-mini
        default: true

    - name: code_generation
        provider_interface: openai
        access_key: $OPENAI_API_KEY
        model: gpt-4o
        usage: Generating new code snippets, functions, or boilerplate based on user prompts or requirements

    - name: code_understanding
        provider_interface: openai
        access_key: $OPENAI_API_KEY
        model: gpt-4.1
        usage: understand and explain existing code snippets, functions, or libraries



.. Note::
    For a complete reference of attributes that you can configure in a prompt target, see :ref:`here <defining_prompt_target_parameters>`.

Route description guide 
-------------------------

The model is trained to perform routing on the following Domain-Action Taxonomy: a two-tier hierarchical structure that separates:
  - **Domains preference (coarse-grain)**: Refers to the high-level category or subject area of the user request, such as healthcare, finance, or coding.
  - **Action preference (fine-grain)**: Specifies the precise task or operation within a given domain, such as appointment booking in healthcare, stock analysis in finance, or bug fixing in coding.

Best practice
-------------------------
- **✅ Consistent Naming:**  Route names should align with their descriptions.

  - ❌ Bad:  
    ```json
    {"name": "math", "description": "handle solving, understanding quadratic equations"}
    ```
  - ✅ Better:  
    ```json
    {"name": "quadratic_equation", "description": "solving and explaining quadratic equations"}
    ```

- **✅ Use Nouns:**  
  Preference-based routing benefits from noun-based descriptions, which provide better semantic coverage.

- **✅ Be Specific:**  Avoid vague or overly broad route definitions.

  - ❌ Bad:  
    ```json
    {"name": "math", "description": "math"}
    ```
  - ✅ Better:  
    ```json
    {"name": "math_concepts", "description": "solving math problems and explaining core math concepts"}
    ```

What we don't support
-------------------------

The following features are **not supported** by the Arch-Router model:

- **❌ Multi-Modality:**  
  The model is not trained to process raw image or audio inputs. While it can handle textual queries *about* these modalities (e.g., "generate an image of a cat"), it cannot interpret encoded multimedia data directly.

- **❌ Function Calling:**  
  This model is designed for **semantic preference matching**, not exact intent classification or tool execution. For structured function invocation, use models in the **Arch-Function-Calling** collection.

- **❌ System Prompt Dependency:**  
  Arch-Router routes based solely on the user’s conversation history. It does not use or rely on system prompts for routing decisions.

Remember, working with LLMs is part science, part art. Don't be afraid to experiment and iterate to find what works best for your specific use case.
