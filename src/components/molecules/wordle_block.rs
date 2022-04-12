use yew::prelude::*;

#[function_component(WordleBlock)]
pub fn wordle_block() -> Html {
    html! {
        <div class="bg-gray-900 flex flex-col gap-8">
            <div class="flex items-center justify-between max-w-xl py-4 mx-auto border-b border-gray-800">
                <div class="flex items-center flex-1 gap-4">
                    <a class="text-sm text-white hover:text-gray-300" href="https://twitter.com/itsmarkmead" target="_blank" rel="noopener noreferrer">{"Twitter"}</a>
                    <a class="text-sm text-white hover:text-gray-300" href="https://www.hyperui.dev/" target="_blank" rel="noopener noreferrer">{"HyperUI"}</a>
                </div>
                <h1 class="flex-1 font-bold text-center text-white">{"Tailwind CSS Wordle"}</h1>
                <div class="flex justify-end flex-1">
                    <div class="relative">
                        <span class="flex">
                            <button><svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg></button>
                        </span>
                    </div>
                </div>
            </div>
            <div class="space-y-1 ">
                <div class="flex gap-1 sm:justify-center">
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                </div>
                <div class="flex gap-1 sm:justify-center">
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                </div>
                <div class="flex gap-1 sm:justify-center">
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                </div>
                <div class="flex gap-1 sm:justify-center">
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                </div>
                <div class="flex gap-1 sm:justify-center">
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                    <div class="w-10 h-10 bg-gray-700 rounded-sm shrink-0"></div>
                </div>
            </div>
            <div class="flex justify-center gap-4 ">
                <button class="p-3 text-sm text-white bg-gray-800 rounded-lg hover:text-gray-300">{"New Class Name"}</button>
                <button class="p-3 text-sm text-white bg-gray-800 rounded-lg hover:text-gray-300">{"Play With a Friend"}</button>
            </div>
            <div class="inset-x-0 py-8  text-white">
                <div class="max-w-xl px-4 mx-auto">
                    <form class="pt-8 mt-8 border-t border-gray-800">
                        <fieldset class="flex gap-4 disabled:opacity-50 disabled:pointer-events-none">
                            <div class="relative flex-1">
                                <input class="w-full py-3 pl-3 pr-12 text-sm bg-gray-800 border border-gray-700 rounded-lg" placeholder="Enter guess" min="5" max="5" value="" spellcheck="false" data-ms-editor="true"/>
                                <span class="absolute text-xs -translate-y-1/2 right-3 top-1/2">{"0/5"}</span>
                            </div>
                            <button class="p-3 text-sm text-white bg-indigo-600 rounded-lg hover:bg-indigo-700" type="submit">{"Submit Guess"}</button>
                        </fieldset>
                    </form>
                </div>
            </div>
        </div>
    }
}
